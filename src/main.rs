mod lib { pub mod query; }
use mini_redis::{client, Result};
use lib::query::query_output;


#[tokio::main]

async fn main() -> Result<()> {
    let mut client = client::connect("localhost:6000").await?;
    add_values().await?;
    /*  Query and Output Results */
    let result_1 = client.get("foo").await?;
    let result_2 = client.get("new").await?;
    let result_3 = client.get("test").await?;
    query_output("foo", result_1).await;
    query_output("new", result_2).await;
    query_output("test", result_3).await;
    Ok(())
}

async fn add_values() -> Result<()> {
    let mut client = client::connect("localhost:6000").await?;
    /* Set Values */
    client.set("test", "val".into()).await?;
    Ok(())
}

