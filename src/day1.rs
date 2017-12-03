
/// Computes the captcha of an input slice
/// where the captcha is defined as
/// SUM a[i] where a[i] = a[step(i) % len(a)]
fn captcha<F>(numbers: &[u32], step: F) -> u32   
    where F: Fn(usize) -> usize  { 
    numbers.iter().enumerate()
        .filter(|&(i,_)| numbers[i] == numbers[step(i) % numbers.len()])
        .map(|(_, x)| x)
        .fold(0, |sum, x| sum + x)
}


/// Computes the captcha of an input slice
/// where the captcha is defined as
/// SUM a[i] where a[i] = a[i+i % len(a)]
pub fn simple_captcha(numbers: &[u32]) -> u32 {
    captcha(numbers, |i| i + 1)
}


/// Computes the captcha of an input slice
/// where the captcha is defined as
/// SUM a[i] where a[i] = a[i+len(a)/2 % len(a)]
pub fn complex_captcha(numbers: &[u32]) -> u32 {
    let len_numbers = numbers.len();
    captcha(numbers, |i| i + len_numbers / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test1() {
        let a = [1, 1, 2, 2];
        let c = simple_captcha(&a);
        assert_eq!(c, 3);
    }

    #[test]
    fn simple_test2() {
        let a = [1, 1, 1, 1];
        let c = simple_captcha(&a);
        assert_eq!(c, 4);
    }

    #[test]
    fn simple_test3() {
        let a = [1, 2, 3, 4];
        let c = simple_captcha(&a);
        assert_eq!(c, 0);
    }

    #[test]
    fn simple_test4() {
        let a = [9, 1, 2, 1, 2, 1, 2, 9];
        let c = simple_captcha(&a);
        assert_eq!(c, 9);
    }

    #[test]
    fn complex_test1() {
        let a = [1, 2, 1, 2];
        let c = complex_captcha(&a);
        assert_eq!(c, 6);
    }

    #[test]
    fn complex_test2() {
        let a = [1, 2, 2, 1];
        let c = complex_captcha(&a);
        assert_eq!(c, 0);
    }

    #[test]
    fn complex_test3() {
        let a = [1, 2, 3, 4, 2, 5];
        let c = complex_captcha(&a);
        assert_eq!(c, 4);
    }

    #[test]
    fn complex_test4() {
        let a = [1, 2, 3, 1, 2, 3];
        let c = complex_captcha(&a);
        assert_eq!(c, 12);
    }

    #[test]
    fn complex_test5() {
        let a = [1, 2, 1, 3, 1, 4, 1, 5];
        let c = complex_captcha(&a);
        assert_eq!(c, 4);
    }
}


