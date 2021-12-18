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
struct User {
    id: u64,
    name: String,
}

pub async fn add_user(
    redis_pool: Arc<RedisPool>,
    user: User,
) -> Result<usize> {
    let mut conn = redis_pool.get().await.unwrap();
    let res: usize = cmd("SADD")
        .arg("Users")
        .arg(&user)
        .query_async(&mut conn)
        .await?;

    Ok(res)
}

pub async fn get_users(
    redis_pool: Arc<RedisPool>,
) -> Result<Vec<VNodeClientInfo>> {
    let mut conn = redis_pool.get().await.unwrap();
    let res: Vec<User> = cmd("SMEMBERS").arg("Users").query_async(&mut conn).await?;

    Ok(res)
}
```
