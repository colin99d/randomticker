FROM rust:1.75.0 as base

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /app

COPY . .

# BUILD IMAGE

FROM base as build

WORKDIR /app

RUN cargo build --release

# PRODUCTION IMAGE

FROM gcr.io/distroless/cc-debian11

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY --from=build /app/target/release/randomticker /usr/local/bin/randomticker

WORKDIR /usr/local/bin

EXPOSE 8000

CMD ["randomticker"]
