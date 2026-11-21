FROM rust:1-slim-trixie AS build
WORKDIR /app
COPY . .
RUN cargo build --release --locked

FROM debian:trixie-slim
RUN useradd --system --uid 10001 app && mkdir /data && chown app /data
COPY --from=build /app/target/release/rust-axum-api /usr/local/bin/rust-axum-api
USER app
ENV DATABASE_URL=sqlite:///data/todos.db ADDR=0.0.0.0:3000
EXPOSE 3000
VOLUME /data
CMD ["rust-axum-api"]
