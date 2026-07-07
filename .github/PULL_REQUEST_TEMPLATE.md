<!--
Title this PR with Conventional Commits, e.g. `feat(viewer): add latency column to Diagnostics`.
Keep the PR focused — one logical change per PR.
-->

## Summary

<!-- What does this PR change and why? Link the motivation, not just the diff. -->

## Changes

<!-- Bullet the notable changes. Call out anything reviewers should look at first. -->

-

## Related

<!-- Linear issue, ADR, or GitHub issue. e.g. Closes #123, REW-456 -->

-

## Checklist

- [ ] PR title follows [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] `cargo fmt --check` is clean
- [ ] `cargo clippy --all-targets -- -D warnings` is clean (native)
- [ ] `cargo build --lib --target wasm32-unknown-unknown --no-default-features` builds (wasm)
- [ ] `cargo test` passes
- [ ] Public items have `///` docs

## Breaking changes

<!-- Leave blank if none. Call out changes to the gRPC ports, CLI flags, or the bridge/viewer protocol. -->
