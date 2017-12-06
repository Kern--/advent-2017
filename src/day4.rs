use std::collections::HashSet;

/// Determines whether or not a passphrase is valid where validity is defined by each word being unique
fn is_valid_passphrase<'a, Line, Word>(passphrase: Line) -> bool 
    where Word: Into<String> + Clone,
          Line: IntoIterator<Item = Word>
{
    let mut words = HashSet::<String>::new();
    for word in passphrase.into_iter() {
        let clone = word.clone().into();
        if words.contains(&clone) {
            return false;
        }
        words.insert(clone);
    } 
    return true;
}

/// Computes the number of valid passphrases where `is_valid` determines whether any particular line in a Doc is valid
fn compute_num_valid_passphrases<'a, Doc, Line, Word, F>(passphrases: Doc, is_valid: F) -> u32
    where Word: Into<String> + Clone,
          Line: IntoIterator<Item = Word>,
          Doc: IntoIterator<Item = Line>,
          F: Fn(Line) -> bool
{
    passphrases.into_iter().map(is_valid).fold(0, |sum, current| if current { sum + 1 } else { sum })
}

/// Computes the number of valid passphrases where validity is defined as each word in a line is unique
pub fn compute_num_valid_simple_passphrases<'a, Doc, Line, Word>(passphrases: Doc) -> u32
    where Word: Into<String> + Clone,
          Line: IntoIterator<Item = Word>,
          Doc: IntoIterator<Item = Line>
{
    compute_num_valid_passphrases(passphrases, is_valid_passphrase)
}

/// Computes the number of valid passphrases where validity is defined as each word in a line is unique and not an anagram of any other word
pub fn compute_num_valid_complex_passphrases<'a, Doc, Line, Word>(passphrases: Doc) -> u32
    where Word: Into<String> + Clone,
          Line: IntoIterator<Item = Word>,
          Doc: IntoIterator<Item = Line>
{
    // Valid can be defined by first sorting the letters in a word and then using the same valid method as the simple case
    fn is_valid<'a, Line, Word>(words: Line) -> bool
        where Word: Into<String> + Clone,
              Line: IntoIterator<Item = Word> {
        let sorted_words = words.into_iter().map(|word| {
            let string: String = word.clone().into();
            let mut sorted_word = string.chars().collect::<Vec<char>>();
            sorted_word.sort();
            sorted_word.into_iter().collect::<String>()
        }).collect::<Vec<String>>();
        is_valid_passphrase(sorted_words)
    };
    compute_num_valid_passphrases(passphrases, is_valid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_passphrase_1() {
        let passphrase = vec!["aa", "bb", "cc", "dd", "ee"];
        assert!(is_valid_passphrase(passphrase));
    }

    #[test]
    fn test_passphrase_2() {
        let passphrase = vec!["aa", "bb", "cc", "dd", "aa"];
        assert!(!is_valid_passphrase(passphrase));
    }

    #[test]
    fn test_passphrase_3() {
        let passphrase = vec!["aa", "bb", "cc", "dd", "aaa"];
        assert!(is_valid_passphrase(passphrase));
    }

    #[test]
    fn test_simple_passphrases() {
        let passphrases = vec![vec!["aa", "bb", "cc", "dd", "ee"], // yes
            vec!["aa", "bb", "cc", "dd", "aa"], // no
            vec!["aa", "bb", "cc", "dd", "aaa"]]; // yes
        assert_eq!(compute_num_valid_simple_passphrases(passphrases), 2);
    }

    #[test]
    fn test_complex_passphrases() {
        let passphrases = vec![vec!["abcde", "fghij"], // yes
            vec!["abcde", "xyz", "ecdab"], // no
            vec!["a", "ab", "abc", "abd", "abf", "abj"], // yes 
            vec!["iiii", "oiii", "ooii", "oooi", "oooo"], // yes
            vec!["oiii", "ioii", "iioi", "iiio"]]; // no
        assert_eq!(compute_num_valid_complex_passphrases(passphrases), 3);
    }
}