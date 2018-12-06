FROM clux/muslrust:1.17.0-nightly-2017-03-17 as builder
LABEL stage=intermediate

COPY . /workspace
RUN set -x \
  && apt-get update \
  && apt-get install -y ca-certificates \
  && update-ca-certificates \
  && cd /workspace \
  && cargo build -p clubdarn-server --release \
  && mv /workspace/target/*/release /out

FROM scratch
COPY --from=builder /out/clubdarn-server /
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000 \
    ROCKET_ENV=production \
    ROCKET_LOG=normal

ENTRYPOINT ["/clubdarn-server"]
EXPOSE 8000

