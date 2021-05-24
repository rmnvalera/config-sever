FROM rust:1.50 as builder
WORKDIR /usr/src/dSTAR-AppConfig
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y
WORKDIR /AppConfig
COPY --from=builder /usr/local/cargo/bin/dSTAR-AppConfig /usr/local/bin/dSTAR-AppConfig
COPY --from=builder /usr/src/dSTAR-AppConfig/conf /AppConfig/conf
COPY --from=builder /usr/src/dSTAR-AppConfig/resurce /AppConfig/resurce
EXPOSE 8080
CMD ["dSTAR-AppConfig"]