# Python Dice Roll API with OpenTelemetry

A simple Bottle API that simulates rolling dice with built-in OpenTelemetry tracing.

## Features

- Roll multiple 6-sided dice
- OpenTelemetry instrumentation
- HTML interface
- Error handling
- Trace exports to OTLP collector

## Setup

```bash
# Install dependencies
pip install -r requirements.txt

# Set collector endpoint (optional)
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4318"

# Run the server
python app.py
```

## API Usage

### Roll Dice

- Endpoint: `/rolldice`
- Method: GET
- Query Parameter: `rolls` (integer, 1-100)
- Example: `http://localhost:8080/rolldice?rolls=3`

### Response Format

```json
{
  "dice": 3,
  "results": [4, 6, 2]
}
```

## Files Structure

- `app.py` - Main application server
- `dice.py` - Dice rolling logic with tracing
- `instrumentation.py` - OpenTelemetry setup
- `public/index.html` - Web interface
- `static/style.css` - Styling

## OpenTelemetry Integration

The application uses OpenTelemetry for tracing:

- Traces dice rolls individually
- Records roll results as span attributes
- Exports traces to configured OTLP endpoint

## Environment Variables

- `OTEL_EXPORTER_OTLP_ENDPOINT`: Collector endpoint (default: http://localhost:4318)
- `APP_HOST`: Host to bind (default: localhost)
- `APP_PORT`: Port to listen on (default: 8080)
