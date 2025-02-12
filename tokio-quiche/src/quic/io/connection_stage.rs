use std::fmt::Debug;
use std::ops::ControlFlow;
use std::time::Instant;

use tokio::sync::mpsc;

use crate::quic::connection::{
    ApplicationOverQuic, HandshakeError, HandshakeInfo, Incoming, QuicConnectionStatsShared,
};
use crate::quic::QuicheConnection;
use crate::QuicResult;

/// Represents the current lifecycle stage of a [quiche::Connection]. Implementors of this trait
/// inform the underlying I/O loop as to how to behave.
///
/// The I/O loop will always handle sending/receiving packets - this trait simply serves to augment
/// its functionality. For example, an established HTTP/3 connection may want its `on_read` to
/// include handing packets off to an [ApplicationOverQuic].
///
/// To prevent borrow checker conflicts, we inject a `qconn` into all methods. This also simplifies
/// state transitions, since the `IoWorker` must maintain ownership over the connection in order to
/// read, gather, and flush from it.
pub trait ConnectionStage: Send + Debug {
    fn on_read<A: ApplicationOverQuic>(
        &mut self,
        _received_packets: bool,
        _qconn: &mut QuicheConnection,
        _ctx: &mut ConnectionStageContext<A>,
    ) -> QuicResult<()> {
        Ok(())
    }

    fn on_flush<A: ApplicationOverQuic>(
        &mut self,
        _qconn: &mut QuicheConnection,
        _ctx: &mut ConnectionStageContext<A>,
    ) -> ControlFlow<QuicResult<()>> {
        ControlFlow::Continue(())
    }

    fn wait_deadline(&mut self) -> Option<Instant> {
        None
    }

    fn post_wait(&self, _qconn: &mut QuicheConnection) -> ControlFlow<QuicResult<()>> {
        ControlFlow::Continue(())
    }
}

/// Global context shared across all [ConnectionStage]s for a given connection
pub struct ConnectionStageContext<A> {
    pub in_pkt: Option<Incoming>,
    pub application: A,
    pub incoming_pkt_receiver: mpsc::Receiver<Incoming>,
    pub stats: QuicConnectionStatsShared,
}

impl<A> ConnectionStageContext<A>
where
    A: ApplicationOverQuic,
{
    // TODO: remove when AOQ::buffer() situation is sorted - that method shouldn't exist
    pub fn buffer(&mut self) -> &mut [u8] {
        self.application.buffer()
    }
}

#[derive(Debug)]
pub struct Handshake {
    pub handshake_info: HandshakeInfo,
}

impl Handshake {
    fn check_handshake_timeout_expired(&self, conn: &mut QuicheConnection) -> QuicResult<()> {
        if self.handshake_info.is_expired() {
            let _ = conn.close(false, quiche::WireErrorCode::ApplicationError as u64, &[]);
            return Err(HandshakeError::Timeout.into());
        }

        Ok(())
    }
}

impl ConnectionStage for Handshake {
    fn on_flush<A: ApplicationOverQuic>(
        &mut self,
        qconn: &mut QuicheConnection,
        _ctx: &mut ConnectionStageContext<A>,
    ) -> ControlFlow<QuicResult<()>> {
        if qconn.is_established() {
            ControlFlow::Break(Ok(()))
        } else {
            ControlFlow::Continue(())
        }
    }

    fn wait_deadline(&mut self) -> Option<Instant> {
        self.handshake_info.deadline()
    }

    fn post_wait(&self, qconn: &mut QuicheConnection) -> ControlFlow<QuicResult<()>> {
        match self.check_handshake_timeout_expired(qconn) {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }
}

#[derive(Debug)]
pub struct RunningApplication;

impl ConnectionStage for RunningApplication {
    fn on_read<A: ApplicationOverQuic>(
        &mut self,
        received_packets: bool,
        qconn: &mut QuicheConnection,
        ctx: &mut ConnectionStageContext<A>,
    ) -> QuicResult<()> {
        if ctx.application.should_act() {
            if received_packets {
                ctx.application.process_reads(qconn)?;
            }

            if qconn.is_established() {
                ctx.application.process_writes(qconn)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Close {
    pub work_loop_result: QuicResult<()>,
}

impl ConnectionStage for Close {}
