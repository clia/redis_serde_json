# redis_serde_json

A derive to store and retrieve JSON values in redis encoded using serde.

## Example

Cargo.toml:

```toml
[dependencies]
redis_serde_json = { git = "https://github.com/clia/redis_serde_json.git" }
```

main.rs:

```rust
use redis_serde_json::RedisJsonValue;

#[derive(RedisJsonValue)]
struct Example {
    id: i32,
    name: String,
}
```
