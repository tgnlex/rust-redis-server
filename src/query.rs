mod lib { pub mod format; }
use lib::format::format_query;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
  let mut client = client::connect("localhost:6000").await?;
  /*  Query and Output Results */
  let result_1 = client.get("foo").await?;
  let result_2 = client.get("new").await?;
  let result_3 = client.get("test").await?;
  format_query(1, "foo", result_1).await;
  format_query(2, "new", result_2).await;
  format_query(3, "test", result_3).await;
  Ok(())
}


