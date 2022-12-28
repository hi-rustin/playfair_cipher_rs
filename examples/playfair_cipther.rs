//! A simple example of using the playfair_cipher_rs crate.
//! This example uses the Playfair Cipher to encrypt a message.
//! The key is "playfair example" and the plaintext is "Hide the gold in the tree stump".
//! The ciphertext should be "BMODZBXDNABEKUDMUIXMMOUVIF".
//! The ciphertext is printed to the console.
//! This example is based on the example in the Wikipedia article on the Playfair Cipher.
//! https://en.wikipedia.org/wiki/Playfair_cipher

use playfair_cipher_rs::PlayfairCipther;
fn main() {
    let key = "playfair example";
    let plaintext = "Hide the gold in the tree stump";
    let playfair_cipher = PlayfairCipther::new(key.to_string());
    let ciphertext = playfair_cipher.encrypt(plaintext.to_string());
    println!("Ciphertext: {}", ciphertext);
}
