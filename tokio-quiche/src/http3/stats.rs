use std::fmt;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use crossbeam::atomic::AtomicCell;
use datagram_socket::StreamClosureKind;

/// Stream-level HTTP/3 audit statistics recorded by [H3Driver](crate::http3::driver::H3Driver).
pub struct H3AuditStats {
    /// The stream ID of this session.
    stream_id: u64,
    /// The number of bytes sent over the stream.
    downstream_bytes_sent: AtomicU64,
    /// The number of bytes received over the stream.
    downstream_bytes_recvd: AtomicU64,
    /// A STOP_SENDING error code received from the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    recvd_stop_sending_error_code: AtomicI64,
    /// A RESET_STREAM error code received from the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    recvd_reset_stream_error_code: AtomicI64,
    /// A STOP_SENDING error code sent to the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    sent_stop_sending_error_code: AtomicI64,
    /// A RESET_STREAM error code sent to the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    sent_reset_stream_error_code: AtomicI64,
    /// Stream FIN received from the peer.
    recvd_stream_fin: AtomicCell<StreamClosureKind>,
    /// Stream FIN sent to the peer.
    sent_stream_fin: AtomicCell<StreamClosureKind>,
}

impl H3AuditStats {
    pub fn new(stream_id: u64) -> Self {
        Self {
            stream_id,
            downstream_bytes_sent: AtomicU64::new(0),
            downstream_bytes_recvd: AtomicU64::new(0),
            recvd_stop_sending_error_code: AtomicI64::new(-1),
            recvd_reset_stream_error_code: AtomicI64::new(-1),
            sent_stop_sending_error_code: AtomicI64::new(-1),
            sent_reset_stream_error_code: AtomicI64::new(-1),
            recvd_stream_fin: AtomicCell::new(StreamClosureKind::None),
            sent_stream_fin: AtomicCell::new(StreamClosureKind::None),
        }
    }

    /// The stream ID of this session.
    #[inline]
    pub fn stream_id(&self) -> u64 {
        self.stream_id
    }

    /// The number of bytes sent over the stream.
    #[inline]
    pub fn downstream_bytes_sent(&self) -> u64 {
        self.downstream_bytes_sent.load(Ordering::SeqCst)
    }

    /// The number of bytes received over the stream.
    #[inline]
    pub fn downstream_bytes_recvd(&self) -> u64 {
        self.downstream_bytes_recvd.load(Ordering::SeqCst)
    }

    /// A STOP_SENDING error code received from the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    #[inline]
    pub fn recvd_stop_sending_error_code(&self) -> i64 {
        self.recvd_stop_sending_error_code.load(Ordering::SeqCst)
    }

    /// A RESET_STREAM error code received from the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    #[inline]
    pub fn recvd_reset_stream_error_code(&self) -> i64 {
        self.recvd_reset_stream_error_code.load(Ordering::SeqCst)
    }

    /// A STOP_SENDING error code sent to the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    #[inline]
    pub fn sent_stop_sending_error_code(&self) -> i64 {
        self.sent_stop_sending_error_code.load(Ordering::SeqCst)
    }

    /// A RESET_STREAM error code sent to the peer.
    ///
    /// -1 indicates that this error code was not received yet.
    #[inline]
    pub fn sent_reset_stream_error_code(&self) -> i64 {
        self.sent_reset_stream_error_code.load(Ordering::SeqCst)
    }

    /// Stream FIN received from the peer.
    #[inline]
    pub fn recvd_stream_fin(&self) -> StreamClosureKind {
        self.recvd_stream_fin.load()
    }

    /// Stream FIN sent to the peer.
    #[inline]
    pub fn sent_stream_fin(&self) -> StreamClosureKind {
        self.sent_stream_fin.load()
    }

    #[inline]
    pub fn add_downstream_bytes_sent(&self, bytes_sent: u64) {
        self.downstream_bytes_sent
            .fetch_add(bytes_sent, Ordering::SeqCst);
    }

    #[inline]
    pub fn add_downstream_bytes_recvd(&self, bytes_recvd: u64) {
        self.downstream_bytes_recvd
            .fetch_add(bytes_recvd, Ordering::SeqCst);
    }

    #[inline]
    pub fn set_recvd_stop_sending_error_code(&self, recvd_stop_sending_error_code: i64) {
        self.recvd_stop_sending_error_code
            .store(recvd_stop_sending_error_code, Ordering::SeqCst);
    }

    #[inline]
    pub fn set_recvd_reset_stream_error_code(&self, recvd_reset_stream_error_code: i64) {
        self.recvd_reset_stream_error_code
            .store(recvd_reset_stream_error_code, Ordering::SeqCst);
    }

    #[inline]
    pub fn set_sent_stop_sending_error_code(&self, sent_stop_sending_error_code: i64) {
        self.sent_stop_sending_error_code
            .store(sent_stop_sending_error_code, Ordering::SeqCst);
    }

    #[inline]
    pub fn set_sent_reset_stream_error_code(&self, sent_reset_stream_error_code: i64) {
        self.sent_reset_stream_error_code
            .store(sent_reset_stream_error_code, Ordering::SeqCst);
    }

    #[inline]
    pub fn set_recvd_stream_fin(&self, recvd_stream_fin: StreamClosureKind) {
        self.recvd_stream_fin.store(recvd_stream_fin);
    }

    #[inline]
    pub fn set_sent_stream_fin(&self, sent_stream_fin: StreamClosureKind) {
        self.sent_stream_fin.store(sent_stream_fin);
    }
}

impl fmt::Debug for H3AuditStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("H3AuditStats")
            .field("stream_id", &self.stream_id)
            .field("downstream_bytes_sent", &self.downstream_bytes_sent)
            .field("downstream_bytes_recvd", &self.downstream_bytes_recvd)
            .field(
                "recvd_stop_sending_error_code",
                &self.recvd_stop_sending_error_code,
            )
            .field(
                "recvd_reset_stream_error_code",
                &self.recvd_reset_stream_error_code,
            )
            .field(
                "sent_stop_sending_error_code",
                &self.sent_stop_sending_error_code,
            )
            .field(
                "sent_reset_stream_error_code",
                &self.sent_reset_stream_error_code,
            )
            .finish()
    }
}
