impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() { return false; }

        let mut y = x;
        let mut z = 0;

        while y != 0 {
            z *= 10;
            z += y % 10;
            y /= 10;
        }

        x == z
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
