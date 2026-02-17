# 1. Build Frontend

FROM node:20-alpine AS frontend

WORKDIR /build

RUN apk add --no-cache git \
 && git clone --depth 1 https://github.com/arg274/cambia.git .

WORKDIR /build/web

RUN npm ci \
 && npm run build

# 2. Build Backend

FROM rust:alpine AS backend

RUN apk add --no-cache musl-dev pkgconf openssl-dev openssl-libs-static

WORKDIR /build

COPY --from=frontend /build .

RUN cargo fetch

RUN cargo build --release

# 3. Runtime

FROM alpine:3.21 AS runtime

COPY --from=backend /build/target/release/cambia /usr/local/bin/cambia

ENV CAMBIA_SERVER=true
ENV CAMBIA_TRACING=info

EXPOSE 3030

ENTRYPOINT ["cambia"]
