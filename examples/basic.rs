use cerberus_api::{
  client::KeyManagerClient,
  ListKeysRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
  let mut client = KeyManagerClient::connect("http://[::1]:50051").await?;

  let request = tonic::Request::new(ListKeysRequest {});

  let response = client.list_keys(request).await?;

  println!("RESPONSE={:?}", response);

  Ok(())
}