<h1 align="center">
  <a href="https://rewire.run/">
    <img alt="banner" src="https://github.com/user-attachments/assets/4859413d-89b2-424c-a378-8a15260de384">
  </a>
</h1>

<p align="center">
  <a href="https://github.com/rewire-run/viewer/actions/workflows/ci.yaml">
    <img alt="CI" src="https://github.com/rewire-run/viewer/actions/workflows/ci.yaml/badge.svg">
  </a>
  <a href="https://github.com/rewire-run/viewer/releases/latest">
    <img alt="Version" src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Frewire-run%2Fviewer%2Fmain%2FCargo.toml&query=%24.package.version&prefix=v&label=version&color=green">
  </a>
  <a href="https://github.com/rewire-run/viewer/blob/main/LICENSE">
    <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
  </a>
  <a href="https://pixi.sh">
    <img alt="Powered by" src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/prefix-dev/pixi/main/assets/badge/v0.json">
  </a>
</p>

A custom [Rerun](https://rerun.io) viewer for ROS 2 visualization, built on top of Rerun v0.34.

## Features

- **Topics Panel** — sortable table of subscribed ROS 2 topics with type, publisher count, and subscriber count
- **Nodes Panel** — sortable table of discovered ROS 2 nodes with transport info
- **Status Bar** — live connection state, relay-sourced bridge count, node count, topic count, and uptime
- **Self-healing connection** — dials a relay (or any Rerun gRPC endpoint) and auto-reconnects with
  backoff; start order does not matter

## Architecture

Since v0.6.0 the viewer is a pure client — it hosts no servers. It connects to a
[rewire](https://github.com/rewire-run/bridge) relay (`rewire serve`, or the relay the bridge embeds
in its local auto-detect flow) for the Rerun data stream, and polls the relay's
[`rewire.v2.RelayService`](https://github.com/rewire-run/extras/blob/main/proto/rewire/v2/rewire.proto)
on the same port for fleet status.

## Build

Requires Rust 1.82+.

```bash
cargo build --release
```

Or with [pixi](https://pixi.sh):

```bash
pixi run build
pixi run sanity   # check + fmt + lint + test
```

## Run

```bash
cargo run --release
```

By default the viewer connects to `rerun+http://127.0.0.1:9876/proxy`. Point it elsewhere with
`--connect`, which accepts `host`, `host:port`, or a full proxy URL:

```bash
rewire-viewer --connect robot.local:9876
```

If no relay is running yet, the viewer waits in "Connecting..." and attaches as soon as one
appears; if the relay restarts, it reconnects on its own.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
