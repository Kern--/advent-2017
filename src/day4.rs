use std::collections::HashSet;


pub fn is_valid_passphrase<'a, W>(passphrase: W) -> bool 
    where W: IntoIterator<Item = &'a str>
{
    let mut words = HashSet::<String>::new();
    for word in passphrase.into_iter() {
        if words.contains(word.clone()) {
            return false;
        }
        words.insert(word.clone().into());
    } 
    return true;
}

pub fn num_valid_passphrases<'a, S, W>(passphrases: S) -> u32
    where S: IntoIterator<Item = W>,
          W: IntoIterator<Item = &'a str>
{
    passphrases.into_iter().map(is_valid_passphrase).fold(0, |sum, current| if current { sum + 1 } else { sum })
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
}