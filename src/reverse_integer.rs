impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res = (0i32, false);
        let mut x = x;

        while x != 0 {
            let r = x % 10;
            x /= 10;

            res = res.0.overflowing_mul(10);
            if res.1 { return 0; }

            res = res.0.overflowing_add(r);
            if res.1 { return 0; }
        }
        res.0
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
