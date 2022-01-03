impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut sToT = HashMap::new();
        let mut tToS = HashMap::new();
        for (s1,t1) in s.chars().zip(t.chars()) {
            if !sToT.contains_key(&s1) && !tToS.contains_key(&t1) {
                sToT.insert(s1,t1);
                tToS.insert(t1,s1);
            } 
            let sc = sToT.get(&s1); 
            let tc = tToS.get(&t1);
            match (sc,tc) {
                (None,None) => return false,
                (None, _) => return false,
                (_, None) => return false,
                (Some(x), Some(y)) => if *x != t1 && *y != s1 { return false},
            }
        }
        true
    }
}
