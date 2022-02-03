FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY ./Rocket.toml ./Rocket.toml
COPY --from=builder /usr/local/cargo/bin/webserver /usr/local/bin/webserver
CMD ["webserver"]