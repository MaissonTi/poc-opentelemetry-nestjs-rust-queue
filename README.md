# Initialization

### Docker

```
docker-compose up --build -d
```

### Nest

```
pnpm run prisma:init
pnpm run start:dev
```

### Rust

```
cargo build && RUST_LOG=info cargo run
```

#### Clear jaeger

```
docker-compose stop jaeger && docker-compose rm -f jaeger && docker-compose up -d jaeger
```
