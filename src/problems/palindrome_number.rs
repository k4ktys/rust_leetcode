pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let mut n = x;
        let mut reversed = 0;

        while n > 0 {
            reversed = reversed * 10 + (n % 10);
            n = n / 10;
        }

        if reversed == x {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false)
    }
}