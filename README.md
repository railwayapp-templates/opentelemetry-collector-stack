# Open Telemetry Collector on Railway

One-click Railway Template to deploy an [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/) along with Zipkin, Jaeger, and Prometheus.

[![Deploy on Railway](https://railway.app/button.svg)]()

This repository contains the Dockerfile and config yaml for the [Open Telemetry Collector](https://github.com/open-telemetry/opentelemetry-collector/tree/main), ready to deploy to Railway.

Port 4318 is mapped from the public network, so that you can send Traces/Metrics over HTTP frm the public network (ie.. client side apps) directly to the public domain on Railway.

If you are sending data to it from a server side app, you can use the private network to the appropriate port.  This is the port map:

    - "1888"   # pprof extension
    - "8888"   # Prometheus metrics exposed by the collector
    - "8889"   # Prometheus exporter metrics
    - "13133" # health_check extension
    - "4317"   # OTLP gRPC receiver
    - "4318"   # OTLP HTTP receiver
    - "55679" # zpages extension


## accepting data from services outside the Railway private network

If you need to send data to it from the public network, you must keep the PORT env var configured in the service settings to 4318.  If you require access to the zpages extension over PORT 55679, you can update the PORT setting to this value, but you will no longer be able to receive data from over the public network.


## zpages extension

The zpages extension enables an HTTP endpoint that provides live data for debugging different components. -> https://github.com/open-telemetry/opentelemetry-collector/blob/main/extension/zpagesextension/README.md

The zpages extension is enabled, but inaccessible with the default setup since the public port is mapped to 4318 in order to enable the collector to receive metrics from a service outside of the Railway private network.

