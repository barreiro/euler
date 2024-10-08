// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::io::load_default_data;
use crate::Solver;

const KEY_SIZE: usize = 3;

/// Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange).
/// For example, uppercase `A = 65`, `asterisk (*) = 42`, and `lowercase k = 107`.
///
/// A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value, taken from a secret key.
/// The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, `65 XOR 42 = 107`, then `107 XOR 42 = 65`.
///
/// For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes.
/// The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.
///
/// Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key.
/// If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message
/// The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.
///
/// Your task has been made easy, as the encryption key consists of three lower case characters.
/// Using `p059_cipher.txt` (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the original text.
pub struct Solver059 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver059 {
    fn default() -> Self {
        Self { n: 1455, input: load_default_data(59) }
    }
}

impl Solver for Solver059 {
    fn problem_name(&self) -> &str { "XOR decryption" }

    fn solve(&self) -> i64 {
        let encoded = self.input.split(',').filter_map(|s| s.parse().ok()).collect::<Vec<i64>>();

        // the key maximizes the number of space characters
        let max_spaces = |pos| ('a' as _..='z' as _).max_by_key(|k| encoded.iter().skip(pos).step_by(KEY_SIZE).filter(|&c| c ^ k == ' ' as _).count()).as_i64();
        let key = (0..KEY_SIZE).map(max_spaces).collect::<Vec<_>>();

        encoded.iter().take(self.n).enumerate().map(|(i, c)| c ^ key[i % key.len()]).sum()
    }
}
