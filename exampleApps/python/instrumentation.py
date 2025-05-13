import os
from opentelemetry import trace
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter

# Configuration from environment variables
SERVICE_NAME = os.getenv("SERVICE_NAME", "python-dice-server")
SERVICE_VERSION = os.getenv("SERVICE_VERSION", "0.1.0")
OTEL_EXPORT_ENDPOINT = os.getenv("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4318")

# Create a resource with service information
resource = Resource.create(
    {
        "service.name": SERVICE_NAME,
        "service.version": SERVICE_VERSION,
    }
)

# Configure the trace provider with the resource
trace_provider = TracerProvider(resource=resource)
trace.set_tracer_provider(trace_provider)

# Set up the OTLP exporter for traces
trace_exporter = OTLPSpanExporter(
    endpoint=f"{OTEL_EXPORT_ENDPOINT}/v1/traces",
)

# Configure batch processing of spans
span_processor = BatchSpanProcessor(trace_exporter)
trace.get_tracer_provider().add_span_processor(span_processor)

# Create a tracer for use throughout the application
tracer = trace.get_tracer(__name__)
