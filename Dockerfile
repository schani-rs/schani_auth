from debian:latest

COPY target/release/schani_auth /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/schani_auth"]

