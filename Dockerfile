FROM node:22-bookworm-slim AS web
ARG BUILD_SHA=dev
ENV VITE_BUILD_SHA=${BUILD_SHA}
WORKDIR /build
COPY package.json package-lock.json tsconfig.json vite.config.ts index.html ./
COPY public ./public
COPY src ./src
RUN npm ci && npm run build

# Cargo.lock resolves ICU 2.3, whose MSRV is Rust 1.88. Keep this in sync
# with the documented local build requirement and the Dockerfile regression test.
FROM rust:1.88-bookworm AS server
ARG BUILD_SHA=dev
ENV BUILD_SHA=${BUILD_SHA}
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN groupadd --system app && useradd --system --gid app --home-dir /app app && mkdir -p /app/data && chown -R app:app /app
WORKDIR /app
COPY --from=server /build/target/release/client-catalogue-request /usr/local/bin/client-catalogue-request
COPY --from=web /build/dist ./dist
USER app
ENV PORT=8080 DATA_DIR=/app/data WEB_DIST=/app/dist
EXPOSE 8080
VOLUME ["/app/data"]
CMD ["client-catalogue-request"]
