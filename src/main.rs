use crypt::encrypt_mono;

fn main() {
    let plaintext = "HELLO I am Here. I can't believe it.";
    encrypt_mono(plaintext);
}