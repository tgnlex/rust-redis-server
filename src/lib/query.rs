use ::bytes::Bytes;
pub async fn query_output(name: &str, result: Option<Bytes>) {
  println!("\n");
  println!("#=================================================#");
  println!("[Server]: Beginning Query Of Key: {:?}", name);
  println!("#=================================================#");
  println!("[Server]: Retrieving value from Redis.....");
  println!("#=================================================#");
  println!("[Server]: Value Found!");
  println!("[Server]: Result: {:?}", result);
  println!("#=================================================#");
}