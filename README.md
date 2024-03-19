# Open Telemetry Collector Stack

Deploy this [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/) to Railway along with Zipkin, Jaeger, and Prometheus backends, with the click of a button!

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/7KNDff)

# About this Repo

This repository contains the Dockerfile and config yaml for the [Open Telemetry Collector](https://github.com/open-telemetry/opentelemetry-collector/tree/main), ready to deploy to Railway.

It also contains an [example node app](https://github.com/railwayapp-templates/opentelemetry-collector-stack/tree/main/exampleApp) that is instrumented with [Otel's node SDK](https://opentelemetry.io/docs/languages/js/getting-started/nodejs/).

For information on how to deploy the the collector and app, check out [the tutorial in Railway](https://docs.railway.app/tutorials/deploy-an-otel-collector-stack).

## Technical Details

When you deploy this collector and stack from the Railway template by clicking the button above, each of the following services will be deployed.

### OpenTelemetry Collector

The collector is a vendor-agnostic way to receive, process and export telemetry data.  

It is deployed with a [configuration file](https://github.com/railwayapp-templates/opentelemetry-collector-stack/blob/main/otel-collector-config.yaml) that enables it to send data to the complementary backend services.

The zpages extension is enabled, allowing you to connect to the debug UI from your browser.  More information on the extension can be found [here](https://github.com/open-telemetry/opentelemetry-collector/blob/main/extension/zpagesextension/README.md).

#### Documentation

- [Collector Documentation](https://opentelemetry.io/docs/)
- [Configuration File Documentation](https://opentelemetry.io/docs/collector/configuration/)

Port map for reference:

    - "1888"   # pprof extension
    - "8888"   # Prometheus metrics exposed by the collector
    - "8889"   # Prometheus exporter metrics
    - "13133" # health_check extension
    - "4317"   # OTLP gRPC receiver
    - "4318"   # OTLP HTTP receiver
    - "55679" # zpages extension

### Zipkin

Zipkin is a distributed tracing system.  It receives data from the Otel Collector on port 9411.
- [Zipkin Documentation](https://zipkin.io/)

### Jaeger

Jaeger is a distributed tracing system.  It receives data from the Otel Collector on port 4317.
- [Jaeger Documentation](https://www.jaegertracing.io/docs/1.55/)

### Prometheus

Prometheus is a systems monitoring and alerting toolkit.  It receives data from the Otel Collector on port 8889.
- [Prometheus Documentation](https://prometheus.io/docs/introduction/overview/)

## Example Apps
We will continue to add examples as we engage with our users and work through various implementations.  

If you have an example, we would LOVE to include it.  PRs welcome!!
- [node](/exampleApps/node)
- [rust](/exampleApps/rust)