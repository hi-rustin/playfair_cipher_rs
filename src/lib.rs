//! # playfair_cipher_rs
//! This crate provides a simple implementation of the Playfair Cipher.
//! You can use this crate to encrypt and decrypt messages using the Playfair Cipher.
//! Currently, this crate only supports the English alphabet.
//! Please note that the Playfair Cipher does not support the letter J.
//! The letter J is replaced with the letter I.
//!
//! Please see the [Playfair Cipher](https://en.wikipedia.org/wiki/Playfair_cipher) Wikipedia article for more information.
//! # Examples
//! ```
//! use playfair_cipher_rs::Playfair;
//! let playfair = Playfair::new("playfair example".to_string());
//! let cipher_text = playfair.encrypt("Hide the gold in the tree stump".to_string());
//! assert_eq!(cipher_text, "BMODZBXDNABEKUDMUIXMMOUVIF");
//! ```

use std::fmt;

// ALPHABET is a string containing all the letters of the English alphabet except J.
const ALPHABET: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
// FILLER is a filler letter (eg:X) in the duplicate plaintext letters to separate and regroup them.
const FILLER: char = 'X';
// TABLE_SIZE is the size of the table. The Playfair cipher uses a 5 by 5 table containing a key word or phrase.
const TABLE_SIZE: usize = 5;

/// The Playfair cipher uses a 5 by 5 table containing a key word or phrase.
///
/// 1. If the same letters appear in the plaintext in a group,
///    insert a filler letter (eg:x) in the duplicate plaintext letters to separate and regroup them (eg: balloon is regrouped as ba lx lo on).
/// 2. If only one letter is available when grouping to the last group, the letter x is added.
/// 3. If the plaintext letter is in the same row of the matrix, the next letter to its right is taken as the cipher text.
/// 4. If the plaintext letters are in the same column of the matrix, the next letter below it is taken as the cipher text.
/// 5. If the plaintext letters are in different rows and columns of the matrix,
///    the letters that are in the same row and in the same column as another letter in the same group are cipher text.
///
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Playfair {
    /// key is a word or phrase used to create the table.
    key: String,
    // table is a 5 by 5 table containing a key word or phrase.
    table: [[char; TABLE_SIZE]; TABLE_SIZE],
}

impl fmt::Display for Playfair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "key: {}", self.key)?;
        writeln!(f, "table:")?;
        // Display the table in a 5 by 5 format.
        for row in self.table.iter() {
            for col in row.iter() {
                write!(f, "{col} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Playfair {
    /// Create a new Playfair.
    /// # Examples
    /// ```
    /// use playfair_cipher_rs::Playfair;
    /// let playfair = Playfair::new("playfair example".to_string());
    /// println!("{}", playfair);
    /// ```
    pub fn new(key: String) -> Self {
        Self {
            key: key.clone(),
            table: Self::create_table(key),
        }
    }

    /// Encrypt a plaintext.
    /// # Examples
    /// ```
    /// use playfair_cipher_rs::Playfair;
    /// let playfair = Playfair::new("playfair example".to_string());
    /// let cipher_text = playfair.encrypt("Hide the gold in the tree stump".to_string());
    /// assert_eq!(cipher_text, "BMODZBXDNABEKUDMUIXMMOUVIF");
    /// ```
    pub fn encrypt(&self, plain_text: String) -> String {
        let plain_text = plain_text.to_uppercase().replace(' ', "").replace('J', "I");
        let mut cipher_text = String::new();
        let mut chars = plain_text.chars().peekable();
        while let Some(c1) = chars.next() {
            let c2 = match chars.peek() {
                Some(&c) => c,
                // If only one letter is available when grouping to the last group, the letter X is added.
                None => FILLER,
            };
            // If the same letters appear in the plaintext in a group,
            // insert a filler letter (eg:X) in the duplicate plaintext letters to separate and regroup them.
            if c1 == c2 {
                cipher_text.push_str(&self.encrypt_pair(c1, FILLER));
            } else {
                cipher_text.push_str(&self.encrypt_pair(c1, c2));
                chars.next();
            }
        }
        cipher_text
    }

    /// Decrypt a ciphertext.
    /// # Examples
    /// ```
    /// use playfair_cipher_rs::Playfair;
    /// let playfair = Playfair::new("playfair example".to_string());
    /// let plain_text = playfair.decrypt("BMODZBXDNABEKUDMUIXMMOUVIF".to_string());
    /// assert_eq!(plain_text, "HIDETHEGOLDINTHETREXESTUMP");
    /// ```
    pub fn decrypt(&self, cipher_text: String) -> String {
        let mut plain_text = String::new();
        let mut chars = cipher_text.chars().peekable();
        while let Some(c1) = chars.next() {
            let c2 = chars.next().unwrap();
            plain_text.push_str(&self.decrypt_pair(c1, c2));
        }
        plain_text
    }

    fn decrypt_pair(&self, c1: char, c2: char) -> String {
        let (row1, col1) = self.get_index(c1);
        let (row2, col2) = self.get_index(c2);
        // If two letters are in the same row, take the letter to the left of each letter.
        if row1 == row2 {
            let new_col1 = (col1 + TABLE_SIZE - 1) % TABLE_SIZE;
            let new_col2 = (col2 + TABLE_SIZE - 1) % TABLE_SIZE;
            format!(
                "{}{}",
                self.table[row1][new_col1], self.table[row2][new_col2]
            )
        } else if col1 == col2 {
            // If tow letters are in the same column, take the letter above each letter.
            let new_row1 = (row1 + TABLE_SIZE - 1) % TABLE_SIZE;
            let new_row2 = (row2 + TABLE_SIZE - 1) % TABLE_SIZE;
            format!(
                "{}{}",
                self.table[new_row1][col1], self.table[new_row2][col2]
            )
        } else {
            // If two letters are in different rows and columns,
            // take the letters in the same row and in the same column as the other letter in the pair.
            format!("{}{}", self.table[row1][col2], self.table[row2][col1])
        }
    }

    fn encrypt_pair(&self, c1: char, c2: char) -> String {
        let (row1, col1) = self.get_index(c1);
        let (row2, col2) = self.get_index(c2);
        // If two letters are in the same row, take the letter to the right of each letter.
        if row1 == row2 {
            let new_col1 = (col1 + 1) % TABLE_SIZE;
            let new_col2 = (col2 + 1) % TABLE_SIZE;
            format!(
                "{}{}",
                self.table[row1][new_col1], self.table[row2][new_col2]
            )
        } else if col1 == col2 {
            // If tow letters are in the same column, take the letter below each letter.
            let new_row1 = (row1 + 1) % TABLE_SIZE;
            let new_row2 = (row2 + 1) % TABLE_SIZE;
            format!(
                "{}{}",
                self.table[new_row1][col1], self.table[new_row2][col2]
            )
        } else {
            // If two letters are in different rows and columns,
            // take the letters in the same row and in the same column as the other letter in the pair.
            format!("{}{}", self.table[row1][col2], self.table[row2][col1])
        }
    }

    fn create_table(key: String) -> [[char; TABLE_SIZE]; TABLE_SIZE] {
        // Make sure the key is uppercase and replace J with I, because we are using a 5 by 5 table.
        let key = key.to_uppercase().replace(' ', "").replace('J', "I");
        let mut temp = vec![];
        // Fill the temp with the key.
        for c in key.chars() {
            if !temp.contains(&c) {
                temp.push(c);
            }
        }
        // Fill the temp with the rest of the alphabet.
        for c in ALPHABET.chars() {
            if !temp.contains(&c) {
                temp.push(c);
            }
        }
        // Create the table.
        let mut table = [[' '; TABLE_SIZE]; TABLE_SIZE];
        for i in 0..TABLE_SIZE {
            for j in 0..TABLE_SIZE {
                table[i][j] = temp[i * TABLE_SIZE + j];
            }
        }
        table
    }

    fn get_index(&self, c: char) -> (usize, usize) {
        for i in 0..TABLE_SIZE {
            for j in 0..TABLE_SIZE {
                if self.table[i][j] == c {
                    return (i, j);
                }
            }
        }
        unreachable!("{} is not in the table", c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use the example from https://en.wikipedia.org/wiki/Playfair_cipher
    // to test the Playfair.
    // The table is:
    // P L A Y F
    // I R E X M
    // B C D G H
    // K N O Q S
    // T U V W Z
    const TEST_KEY: &str = "playfair example";

    #[test]
    fn test_display_table() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        assert_eq!(
            playfair.to_string(),
            "key: playfair example\ntable:\nP L A Y F \nI R E X M \nB C D G H \nK N O Q S \nT U V W Z \n"
        );
    }

    #[test]
    fn test_create_table() {
        let table = Playfair::create_table(TEST_KEY.to_string());
        assert_eq!(table[0][0], 'P');
        assert_eq!(table[0][1], 'L');
        assert_eq!(table[0][2], 'A');
        assert_eq!(table[0][3], 'Y');
        assert_eq!(table[0][4], 'F');
        assert_eq!(table[1][0], 'I');
        assert_eq!(table[1][1], 'R');
        assert_eq!(table[1][2], 'E');
        assert_eq!(table[1][3], 'X');
        assert_eq!(table[1][4], 'M');
        assert_eq!(table[2][0], 'B');
        assert_eq!(table[2][1], 'C');
        assert_eq!(table[2][2], 'D');
        assert_eq!(table[2][3], 'G');
        assert_eq!(table[2][4], 'H');
        assert_eq!(table[3][0], 'K');
        assert_eq!(table[3][1], 'N');
        assert_eq!(table[3][2], 'O');
        assert_eq!(table[3][3], 'Q');
        assert_eq!(table[3][4], 'S');
        assert_eq!(table[4][0], 'T');
        assert_eq!(table[4][1], 'U');
        assert_eq!(table[4][2], 'V');
        assert_eq!(table[4][3], 'W');
        assert_eq!(table[4][4], 'Z');
    }

    #[test]
    fn test_get_index() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        assert_eq!(playfair.get_index('P'), (0, 0));
        assert_eq!(playfair.get_index('L'), (0, 1));
        assert_eq!(playfair.get_index('A'), (0, 2));
        assert_eq!(playfair.get_index('Y'), (0, 3));
        assert_eq!(playfair.get_index('F'), (0, 4));
        assert_eq!(playfair.get_index('I'), (1, 0));
        assert_eq!(playfair.get_index('R'), (1, 1));
        assert_eq!(playfair.get_index('E'), (1, 2));
        assert_eq!(playfair.get_index('X'), (1, 3));
        assert_eq!(playfair.get_index('M'), (1, 4));
        assert_eq!(playfair.get_index('B'), (2, 0));
        assert_eq!(playfair.get_index('C'), (2, 1));
        assert_eq!(playfair.get_index('D'), (2, 2));
        assert_eq!(playfair.get_index('G'), (2, 3));
        assert_eq!(playfair.get_index('H'), (2, 4));
        assert_eq!(playfair.get_index('K'), (3, 0));
        assert_eq!(playfair.get_index('N'), (3, 1));
        assert_eq!(playfair.get_index('O'), (3, 2));
        assert_eq!(playfair.get_index('Q'), (3, 3));
        assert_eq!(playfair.get_index('S'), (3, 4));
        assert_eq!(playfair.get_index('T'), (4, 0));
        assert_eq!(playfair.get_index('U'), (4, 1));
        assert_eq!(playfair.get_index('V'), (4, 2));
        assert_eq!(playfair.get_index('W'), (4, 3));
        assert_eq!(playfair.get_index('Z'), (4, 4));
    }

    #[test]
    fn test_encrypt_pair() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        assert_eq!(playfair.encrypt_pair('H', 'E'), "DM");
        assert_eq!(playfair.encrypt_pair('P', 'I'), "IB");
        assert_eq!(playfair.encrypt_pair('A', 'B'), "PD");
        assert_eq!(playfair.encrypt_pair('R', 'M'), "EI");
    }

    #[test]
    fn test_encrypt() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        // The example from https://en.wikipedia.org/wiki/Playfair_cipher
        // is "Hide the gold in the tree stump".
        // The result should be "BMODZBXDNABEKUDMUIXMMOUVIF".
        assert_eq!(
            playfair.encrypt("Hide the gold in the tree stump".to_string()),
            "BMODZBXDNABEKUDMUIXMMOUVIF"
        );
    }

    #[test]
    fn test_decrypt_pair() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        assert_eq!(playfair.decrypt_pair('D', 'M'), "HE");
        assert_eq!(playfair.decrypt_pair('I', 'B'), "PI");
        assert_eq!(playfair.decrypt_pair('P', 'D'), "AB");
        assert_eq!(playfair.decrypt_pair('E', 'I'), "RM");
    }

    #[test]
    fn test_decrypt() {
        let playfair = Playfair::new(TEST_KEY.to_string());
        // The example from https://en.wikipedia.org/wiki/Playfair_cipher
        // is "Hide the gold in the tree stump".
        // The result should be "BMODZBXDNABEKUDMUIXMMOUVIF".
        // We decrypt it back to "HIDETHEGOLDINTHETREXESTUMP".
        assert_eq!(
            playfair.decrypt("BMODZBXDNABEKUDMUIXMMOUVIF".to_string()),
            "HIDETHEGOLDINTHETREXESTUMP"
        );
    }
}
