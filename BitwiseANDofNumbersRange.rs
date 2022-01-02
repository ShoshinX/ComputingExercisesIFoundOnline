pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        // Main problem is to avoid the wraparound edge case, so enclose it inside a bigger sized type such as u64
        let mut sum = left as u64;
        let mut i = (left+1) as u64;
        while i <= right as u64{
            sum = sum & i;
            if sum == 0{
                return 0;
            }
            i += 1;
        }
        sum as i32
    }
