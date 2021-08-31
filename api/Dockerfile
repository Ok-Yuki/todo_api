FROM rust:1.54 AS develop-stage
EXPOSE 8080
WORKDIR /api
RUN apt update
RUN apt install -y libpq-dev
RUN cargo install cargo-watch
COPY . .

FROM develop-stage as build-stage
RUN cargo build --release

FROM debian:10.4
RUN apt update
RUN apt install -y libpq-dev
COPY --from=build-stage /api/target/release/todo_api /usr/local/bin/api
EXPOSE 8080
CMD ["/usr/local/bin/api"]
