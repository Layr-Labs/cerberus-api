use cerberus_api::{
    client::SignerClient,
    SignGenericRequest,
    SignGenericResponse,
};
use ark_bn254::{Fq,g1::G1Affine};
use ark_ff::PrimeField;
use eigen_crypto_bls::BlsG1Point;
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let mut client: SignerClient<tonic::transport::Channel> = SignerClient::connect("http://[::1]:50051").await?;
    let message = "Hello, world!";
    let message_bytes = message.as_bytes();
    let request = tonic::Request::new(SignGenericRequest {
      public_key: "9ad8e58d3419955bef69837039c76443e09477dec1eeff728fddcdcd6a59550b".to_string(),
      data: message_bytes.to_vec(),
      password: "Testacc1Testacc1".to_string(),
    });
    let response: tonic::Response<SignGenericResponse> = client.sign_generic(request).await?;
    let g1_affine = response_to_g1_affine(response.into_inner())?;
    let bls_g1_point = BlsG1Point::new(g1_affine);
    println!("bls_g1_point={:?}", bls_g1_point);
    Ok(())
}

pub fn response_to_g1_affine(response: SignGenericResponse) -> Result<G1Affine,Box<dyn std::error::Error>> {
    let signature_bytes = response.signature.clone();
    Ok(G1Affine::new(Fq::from_be_bytes_mod_order(&signature_bytes[0..32]), Fq::from_be_bytes_mod_order(&signature_bytes[32..64])))
}