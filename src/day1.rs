
/// Computes the captcha of an input slice
/// where the captcha is defined as
/// SUM a[i] where a[i] = a[i+1 % len(a)]
pub fn captcha(numbers: &[u32]) -> u32 {
    numbers.iter().enumerate()
        .filter(|&(i,_)| numbers[i] == numbers[(i+1) % numbers.len()])
        .map(|(_, x)| x)
        .fold(0, |sum, x| sum + x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let a = [1, 1, 2, 2];
        let c = captcha(&a);
        assert_eq!(c, 3);
    }

    #[test]
    fn test2() {
        let a = [1, 1, 1, 1];
        let c = captcha(&a);
        assert_eq!(c, 4);
    }

    #[test]
    fn test3() {
        let a = [1, 2, 3, 4];
        let c = captcha(&a);
        assert_eq!(c, 0);
    }

    #[test]
    fn test4() {
        let a = [9, 1, 2, 1, 2, 1, 2, 9];
        let c = captcha(&a);
        assert_eq!(c, 9);
    }
}


