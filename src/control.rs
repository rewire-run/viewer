//! Loopback host for Rerun's `ViewerControlService`.
//!
//! The Rerun MCP server (`rerun viewer-mcp`) drives a viewer through the
//! `ViewerControlService` gRPC endpoint. Stock Rerun hosts it next to its message
//! proxy; rewire-viewer is a pure relay client and hosts nothing, so this module
//! spawns the service on a loopback-only endpoint instead. Control commands arrive
//! as UI-command entries on the returned receiver, which the app drains like any
//! other data source. The message proxy that comes with the spawn helper is an
//! implementation detail, not a supported ingestion path.

use std::io;
use std::net::{Ipv4Addr, SocketAddr};

use re_grpc_server::{
    shutdown, LoopbackServices, MessageProxyHandle, ServerListener, ServerOptions,
};
use re_log_channel::LogReceiver;

/// Default port for the viewer-control endpoint.
///
/// Port 9876 belongs to the relay, which the viewer dials on the same machine by
/// default, so the control service stays off it.
pub const DEFAULT_PORT: u16 = 9878;

/// Spawns the control service on the loopback interface and returns the proxy
/// handle, which keeps the service alive, plus the receiver carrying its UI
/// commands.
///
/// Prefers [`DEFAULT_PORT`] and falls back to an ephemeral port when it is
/// taken. Must be called from within a tokio runtime.
///
/// # Errors
///
/// Fails when no loopback port can be bound at all.
pub fn spawn() -> io::Result<(MessageProxyHandle, LogReceiver)> {
    let listener = bind()?;
    let addr = listener.local_addr();
    let (rx, proxy) = re_grpc_server::spawn_with_recv_and_services(
        listener,
        ServerOptions::default(),
        shutdown::never(),
        LoopbackServices::default(),
    );
    re_log::info!("Viewer control listening on http://{addr} (loopback only)");
    Ok((proxy, rx))
}

fn bind() -> io::Result<ServerListener> {
    ServerListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, DEFAULT_PORT)))
        .or_else(|_| ServerListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))))
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn bind_prefers_the_default_port() {
        let addr = bind()
            .map(|l| l.local_addr())
            .expect("a loopback port is free");
        assert!(addr.ip().is_loopback());
        assert_ne!(addr.port(), 0);
    }

    #[test]
    fn bind_falls_back_when_the_default_is_taken() {
        let _occupant = TcpListener::bind((Ipv4Addr::LOCALHOST, DEFAULT_PORT));
        let addr = bind()
            .map(|l| l.local_addr())
            .expect("an ephemeral port is free");
        assert!(addr.ip().is_loopback());
        assert_ne!(addr.port(), DEFAULT_PORT);
        assert_ne!(addr.port(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn spawn_returns_a_live_handle() {
        spawn().expect("control service spawns");
    }
}
