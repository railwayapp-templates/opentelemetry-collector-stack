FROM otel/opentelemetry-collector:0.97.0

COPY otel-collector-config.yaml /etc/otel-collector-config.yaml

EXPOSE 1888 8888 8889 13133 4317 4318 55679

CMD ["--config=/etc/otel-collector-config.yaml"]
