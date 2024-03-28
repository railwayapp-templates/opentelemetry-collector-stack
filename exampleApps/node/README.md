# Node Example App with Tracing

This repo contains a simple Node Express app largely copied from [OpenTelemetry's example Node app](https://opentelemetry.io/docs/languages/js/getting-started/nodejs/).

### Details

- [app.js](/exampleApps/node/app.js) - the server implementation is here.
- [dice.js](/exampleApps/node/dice.js) - tracing is implemented here in the downstream functions.
- [instrumentation.js](/exampleApps/node/instrumentation.js) - the instrumentation logic is defined here.  This will wrap the server on startup by issuing a start command like `node --require ./instrumentation.js app.js`

### Helpful Resources

These are some resources we found helpful.
- [OpenTelemetry Docs](https://opentelemetry.io/docs/languages/js/)