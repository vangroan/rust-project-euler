

#[allow(dead_code)]
pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}


#[allow(dead_code)]
pub fn is_palindrome(n: i64) -> bool{
    let mut rev = 0;
    let mut x = n;

    while x > 0 {
        rev = (x % 10) + (rev * 10);
        x /= 10;
    }

    n == rev
}


#[cfg(test)]
mod tests {
    use super::{is_prime, is_palindrome};

    #[test]
    fn test_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
        assert_eq!(is_prime(29), true);
    }

    #[test]
    fn test_not_prime() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(12), false);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(98766789), true);
        assert_eq!(is_palindrome(9876789), true);
    }

    
    #[test]
    fn test_not_is_palindrome() {
        assert_eq!(is_palindrome(987689), false);
        assert_eq!(is_palindrome(12345), false);
    }
}
