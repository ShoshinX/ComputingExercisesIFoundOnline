impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut num = n;
        while (num != 1){
            if set.contains(&num) {
                break;
            }
            set.insert(num);
            num = Solution::sum_of_squares_of_digits(num);
        }
        if num == 1 {
            true
        } else {
            false
        }
    }
    
    fn sum_of_squares_of_digits(n:i32) -> i32 {
        let mut vec = vec![];
        let mut num = n;
        while num > 0 {
            vec.push(num % 10);
            num /= 10;
        }
        vec.iter().fold(0i32, |sum,val| sum+(val*val))
    }
}
