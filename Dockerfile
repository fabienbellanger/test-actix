FROM rust:latest as build

LABEL maintainer="Fabien Bellanger <valentil@gmail.com>"

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app
COPY . .

RUN cargo clean
RUN cargo build
# RUN cargo build --release

# -----------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian10

WORKDIR /app

COPY --from=build /app/target/debug/test-actix .
COPY --from=build /app/.env .
COPY --from=build /app/projects.json .

CMD ["test-actix"]
