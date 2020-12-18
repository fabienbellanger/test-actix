FROM rust:latest as build

LABEL maintainer="Fabien Bellanger <valentil@gmail.com>"

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . .

RUN cargo install --path .

# RUN cargo build --release

# -----------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian10

WORKDIR /app

COPY --from=build /usr/local/cargo/bin/test-actix /usr/local/bin/test-actix
COPY --from=build /app/.env .
COPY --from=build /app/projects.json .

CMD ["test-actix"]
EXPOSE 8089
