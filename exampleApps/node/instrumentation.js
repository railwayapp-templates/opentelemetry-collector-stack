const { NodeSDK } = require('@opentelemetry/sdk-node');
const { getNodeAutoInstrumentations } = require('@opentelemetry/auto-instrumentations-node');
const { OTLPTraceExporter } = require('@opentelemetry/exporter-trace-otlp-proto');
const { Resource } = require('@opentelemetry/resources')
const { SEMRESATTRS_SERVICE_NAME, SEMRESATTRS_SERVICE_VERSION, } = require('@opentelemetry/semantic-conventions')
const { OTLPMetricExporter } = require('@opentelemetry/exporter-metrics-otlp-proto');
const { PeriodicExportingMetricReader } = require('@opentelemetry/sdk-metrics');


const sdk = new NodeSDK({
  resource: new Resource({
    [SEMRESATTRS_SERVICE_NAME]: 'dice-server',
    [SEMRESATTRS_SERVICE_VERSION]: '0.1.0',
  }),
  traceExporter: new OTLPTraceExporter({
    url: `http://${process.env.OTEL_EXPORTER_OTLP_ENDPOINT}/v1/traces`
  }),
  metricReader: new PeriodicExportingMetricReader({
    exporter: new OTLPMetricExporter({
        url: `http://${process.env.OTEL_EXPORTER_OTLP_ENDPOINT}/v1/metrics`
      }),
  }),
  instrumentations: [getNodeAutoInstrumentations()],
});

sdk.start();
