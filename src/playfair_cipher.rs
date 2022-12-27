// ALPHABET is a string containing all the letters of the English alphabet except J.
const ALPHABET: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
// FILLER is a filler letter (eg:Z) in the duplicate plaintext letters to separate and regroup them.
const FILLER: char = 'Z';
// TABLE_SIZE is the size of the table. The Playfair cipher uses a 5 by 5 table containing a key word or phrase.
const TABLE_SIZE: usize = 5;

/// The Playfair cipher uses a 5 by 5 table containing a key word or phrase.
///
/// 1. If the same letters appear in the plaintext in a group,
///    insert a filler letter (eg:z) in the duplicate plaintext letters to separate and regroup them (eg: balloon is regrouped as ba lz lo on).
/// 2. If only one letter is available when grouping to the last group, the letter z is added.
/// 3. If the plaintext letter is in the same row of the matrix, the next letter to its right is taken as the cipher text.
/// 4. If the plaintext letters are in the same column of the matrix, the next letter below it is taken as the cipher text.
/// 5. If the plaintext letters are in different rows and columns of the matrix,
///    the letters that are in the same row and in the same column as another letter in the same group are cipher text.
///
pub(crate) struct PlayfairCipther {
    /// key is a word or phrase used to create the table.
    key: String,
    // table is a 5 by 5 table containing a key word or phrase.
    table: [[char; TABLE_SIZE]; TABLE_SIZE],
}

impl PlayfairCipther {
    /// Create a new PlayfairCipther.
    pub fn new(key: String) -> Self {
        Self {
            key: key.clone(),
            table: Self::create_table(key),
        }
    }

    fn create_table(key: String) -> [[char; TABLE_SIZE]; TABLE_SIZE] {
        // Make sure the key is uppercase and replace J with I, because we are using a 5 by 5 table.
        // TODO: deduplicate the key.
        let key = key.to_uppercase().replace(" ", "").replace("J", "I");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let key = "playfair example".to_string();
        let table = PlayfairCipther::create_table(key);
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
}
