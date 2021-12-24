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
use std::sync::Arc;

use anyhow::Result;
use deadpool_redis::{redis::cmd, Pool as RedisPool};
use redis_serde_json::RedisJsonValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, RedisJsonValue)]
pub struct User {
  pub id: u64,
  pub name: String,
}

pub async fn add_user(
  redis_pool: Arc<RedisPool>,
  user: User,
) -> Result<usize> {
  let mut conn = redis_pool.get().await?;
  let res: usize = cmd("SADD")
    .arg("Users")
    .arg(&user)
    .query_async(&mut conn)
    .await?;

  Ok(res)
}

pub async fn get_users(
  redis_pool: Arc<RedisPool>,
) -> Result<Vec<User>> {
  let mut conn = redis_pool.get().await?;
  let res: Vec<User> = cmd("SMEMBERS").arg("Users").query_async(&mut conn).await?;

  Ok(res)
}
```
