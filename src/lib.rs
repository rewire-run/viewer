//! A custom [Rerun](https://rerun.io) viewer for ROS 2 visualization, built on Rerun v0.41.
//!
//! Rewire Viewer extends the Rerun native viewer with ROS 2-specific panels (Topics, Nodes,
//! Diagnostics) and a status bar showing relay connectivity. It is a pure client: it dials a
//! relay (`rewire serve`) or any Rerun gRPC endpoint for data and auto-reconnects with backoff.
//!
//! Supports both native (desktop) and WebAssembly builds.

/// Wrapper around [`re_viewer::App`] that adds the Rewire status bar.
pub mod app;
/// Reconnecting client connection to a relay.
pub mod connection;
/// Loopback viewer-control service for the Rerun MCP.
#[cfg(not(target_arch = "wasm32"))]
pub mod control;
/// Parsing for the viewer's theme preference.
pub mod theme;
/// Status bar and shared UI helpers.
pub mod ui;
/// Custom Rerun SpaceView classes for ROS 2 data.
pub mod views;

#[cfg(target_arch = "wasm32")]
mod web;

/// Build info handed to [`re_viewer::App`].
///
/// Rerun reads `version` as the Rerun version, for example when saving blueprints, so it
/// carries the version of the Rerun crates this viewer is built on, not the viewer's own.
#[must_use]
pub fn build_info() -> re_build_info::BuildInfo {
    re_build_info::BuildInfo {
        version: re_build_info::CrateVersion::LOCAL,
        ..re_build_info::cargo_build_info!()
    }
}

/// Entry point for the WebAssembly build.
#[cfg(target_arch = "wasm32")]
pub use web::RewireWebHandle;
