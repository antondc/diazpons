FROM node:18-bookworm AS assets

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
RUN npm run build::js


FROM rust:latest AS build

WORKDIR /app

RUN apt-get update -o Acquire::Retries=5 -o Acquire::http::Timeout="30" \
  && apt-get install -y --no-install-recommends \
    pkg-config \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY . .

COPY --from=assets /app/dist ./dist

RUN cargo build --release


FROM rust:latest AS development

WORKDIR /app

RUN apt-get update -o Acquire::Retries=5 -o Acquire::http::Timeout="30" \
  && apt-get install -y --no-install-recommends \
    nodejs npm \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

COPY package.json package-lock.json ./
RUN npm ci

COPY . .

CMD ["npm", "run", "run::dev"]


FROM debian:bookworm-slim AS production

WORKDIR /app

RUN apt-get update -o Acquire::Retries=5 -o Acquire::http::Timeout="30" \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/diazpons ./diazpons
COPY --from=build /app/dist ./dist
COPY --from=build /app/data ./data
COPY --from=build /app/src ./src

CMD ["./diazpons"]
