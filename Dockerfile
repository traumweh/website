FROM rust:latest as builder
WORKDIR /usr/src/mainpage
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /usr/src/mainpage
COPY --from=builder /usr/local/cargo/bin/mainpage /usr/local/bin/mainpage
EXPOSE 8080
CMD ["mainpage"]
