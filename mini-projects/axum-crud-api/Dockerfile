#stage 1
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release


#stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/axum-crud-api .
EXPOSE 8000
CMD ["./axum-crud-api"]