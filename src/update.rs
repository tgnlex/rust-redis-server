use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {  
  println!("\n\n Updating redis data... \n");
  add_values().await?;
  print_closing_message();
  Ok(())
}

async fn add_values() -> Result<()> {
  let mut client = client::connect("localhost:6000").await?;
  /* Set Values */
  client.set("test", "value".into()).await?;
  client.set("foo", "newbar".into()).await?;
  client.set("the", "game".into()).await?;
  client.set("chromosomes", "7".into()).await?;
  Ok(())
}

fn print_closing_message() {
  println!("#===================================================#");
  println!("  [SERVER]: Process complete!");
  println!("  [SERVER]: Please verify data was updated correctly.");
  println!("#===================================================#");
}