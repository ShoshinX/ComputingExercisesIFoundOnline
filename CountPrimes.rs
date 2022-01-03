impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // Use Sieve of Erasthenes algorithm
        if n < 2 { return 0;}
        let mut nums = vec![0;n as usize];
        nums[1] = 1;
        nums[0] = 1;
        let mut p = 2;
        while p < n {
            for incr in (p+p..n).step_by(p as usize)  {
                nums[incr as usize] = 1;
            }
            p+=1;
            while p < n {
                if nums[p as usize] == 0{
                    break;
                }
                p += 1;
            }
        }
        let anss : Vec<bool>= nums.iter().map(|x| if *x == 1 { false } else {true}).filter(|x| *x==true).collect();
        anss.len() as i32
    }
}
