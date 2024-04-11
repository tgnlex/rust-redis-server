use ::bytes::Bytes;
pub async fn format_query(id: i32, name: &str, result: Option<Bytes>) {
  println!("\n");
  println!("               ######################");
  println!("               #   Query Number: {:?}  #", id);
  println!("               ######################");
  println!("\n");
  println!(" #=================================================#");
  println!("    [Server]: Beginning Query Of Key: {:?}", name);
  println!(" #=================================================#");
  println!("    [Server]: Retrieving value from Redis.......");
  println!(" #=================================================#");
  println!("    [Server]: Value Found!");
  println!("    [Server]: Result: {:?}", result);
  println!(" #=================================================#");
}