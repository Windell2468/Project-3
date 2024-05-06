// tests/test_posting.rs

use your_library_name::send_contents;

#[test]
fn test_send_contents() {
    // Create mock data
    let content = vec![1, 2, 3];
    let url = "http://example.com/upload";

    // Call the function and check the result
    match send_contents(content, url) {
        Ok(_) => println!("Content sent successfully"),
        Err(err) => eprintln!("Failed to send content: {}", err),
    }
}
