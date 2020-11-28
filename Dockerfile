FROM rust:1.48 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/test-actix
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/test-actix /usr/local/bin/test-actix

CMD ["test-actix"]
