# Rust Example App with Tracing

This repo contains a simple Rust app largely copied from [OpenTelemetry's example Rust app](https://opentelemetry.io/docs/languages/rust/getting-started/), updated to use the [Tracing crate](https://crates.io/crates/tracing) directly.

### Details

- [instrumentation.rs](/exampleApps/rust/src/instrumentation.rs) - the instrumentation logic is defined here and can be imported into any module throughout the app
- [main.rs](/exampleApps/rust/src/main.rs) - the server implementation is here.  Take note of the usage of the `tracing::instrument`, `tracing::info`, and `tracing::error` macros.  These are used to handle tracing data for the `handle` function.

### Helpful Resources

These are some resources we found helpful.
- [OpenTelemetry Docs](https://opentelemetry.io/docs/languages/rust/)
- [Example App in Rust Otel Community repo](https://github.com/open-telemetry/opentelemetry-rust/tree/main/opentelemetry-otlp/examples/basic-otlp-http)
- [Are we observable yet? Rust Telemetry](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)