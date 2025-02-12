use crate::fixtures::*;
use h3i_fixtures::received_status_code_on_stream;

use foundations::telemetry::{with_test_telemetry, TestTelemetryContext};
use futures::StreamExt;
use futures_util::future::try_join_all;
use std::time::Duration;
use tokio::time::timeout;
use tokio_quiche::metrics::DefaultMetrics;
use tokio_quiche::quic::SimpleConnectionIdGenerator;
use tokio_quiche::settings::{Hooks, TlsCertificatePaths};
use tokio_quiche::{listen, ConnectionParams, InitialQuicConnection};

pub mod async_callbacks;
pub mod connection_close;
pub mod timeouts;

#[tokio::test]
async fn echo() {
    const CONN_COUNT: usize = 5;

    let req_count = |conn_num| conn_num * 100;
    let (url, hook) = start_server();
    let mut reqs = vec![];

    for i in 1..=CONN_COUNT {
        let url = format!("{url}/{i}");

        reqs.push(request(url, req_count(i) as u64))
    }

    let res = try_join_all(reqs).await.unwrap();
    let res_map = map_responses(res);

    assert_eq!(res_map.len(), CONN_COUNT);

    for i in 1..=CONN_COUNT {
        let resps = res_map.get(&i).unwrap();

        assert_eq!(resps.len(), req_count(i));
    }

    assert!(hook.was_called());
}

#[tokio::test]
async fn e2e() {
    let (url, hook) = start_server();
    let url = format!("{url}/1");

    let res = request(url, 1).await.unwrap();
    let res_map = map_responses(vec![res]);

    assert_eq!(res_map.len(), 1);

    let resps = res_map.get(&1).unwrap();
    assert_eq!(resps.len(), 1);
    assert!(hook.was_called());
}

#[tokio::test]
async fn e2e_client_ip_validation_disabled() {
    let quic_settings = QuicSettings {
        max_recv_udp_payload_size: 1400,
        max_send_udp_payload_size: 1400,
        max_idle_timeout: Some(Duration::from_secs(5)),
        disable_client_ip_validation: true,
        ..Default::default()
    };
    let hook = TestConnectionHook::new();

    let url = start_server_with_settings(
        quic_settings,
        Http3Settings::default(),
        hook.clone(),
        handle_connection,
    );
    let url = format!("{url}/1");
    let reqs = vec![request(url, 1)];

    let res = try_join_all(reqs).await.unwrap();
    let res_map = map_responses(res);

    assert_eq!(res_map.len(), 1);

    let resps = res_map.get(&1).unwrap();
    assert_eq!(resps.len(), 1);
    assert!(hook.was_called());
}

#[with_test_telemetry(tokio::test)]
async fn quiche_logs_forwarded_server_side(cx: TestTelemetryContext) {
    let quic_settings = QuicSettings {
        capture_quiche_logs: true,
        ..QuicSettings::default()
    };
    let hook = TestConnectionHook::new();

    let url = start_server_with_settings(
        quic_settings,
        Http3Settings::default(),
        hook,
        handle_connection,
    );
    let url = format!("{url}/1");
    let reqs = vec![request(url, 1)];

    let res = try_join_all(reqs).await.unwrap();
    let res_map = map_responses(res);

    assert_eq!(res_map.len(), 1);

    // Unfortunately, the Foundations `fields` struct is empty for some reason. This is a bit of
    // a hacky test, but it checks for a string that should come from Quiche's Trace logs
    assert!(cx
        .log_records()
        .iter()
        .any(
            |record| (record.message.contains("rx pkt") || record.message.contains("tx pkt"))
                && record.level.as_str() == "TRACE"
        ));
}

#[tokio::test]
async fn test_ioworker_state_machine_pause() {
    let socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let url = format!("http://127.0.0.1:{}", socket.local_addr().unwrap().port());

    let tls_cert_settings = TlsCertificatePaths {
        cert: &path_relative_to_manifest_dir("./certs/proxy-cert.pem"),
        private_key: &path_relative_to_manifest_dir("./certs/proxy-key.pem"),
        kind: tokio_quiche::settings::CertificateKind::X509,
    };

    let hooks = Hooks {
        connection_hook: Some(TestConnectionHook::new()),
    };

    let params = ConnectionParams::new_server(QuicSettings::default(), tls_cert_settings, hooks);
    let mut stream = listen(
        vec![socket],
        params,
        SimpleConnectionIdGenerator,
        DefaultMetrics,
    )
    .unwrap()
    .remove(0);

    tokio::spawn(async move {
        loop {
            let (h3_driver, h3_controller) = ServerH3Driver::new(Http3Settings::default());
            let conn = stream.next().await.unwrap().unwrap();

            let (quic_connection, worker) =
                conn.handshake(h3_driver).await.expect("handshake failed");

            InitialQuicConnection::resume(worker);

            let h3_over_quic = ServerH3Connection::new(quic_connection, h3_controller);
            tokio::spawn(async move {
                handle_connection(h3_over_quic).await;
            });
        }
    });

    let url = format!("{url}/1");
    let summary = timeout(Duration::from_secs(1), h3i_fixtures::request(&url, 1))
        .await
        .expect("request timed out")
        .expect("request failed");

    assert!(received_status_code_on_stream(&summary, 0, 200));
}
