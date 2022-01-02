// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut bfs = vec![];
        
        match root{
            Some(x) => bfs.push(x),
            None => ()
        }
        while bfs.len() > 0 {
            ans.push(bfs[bfs.len() - 1].borrow().val);
            let mut row = vec![];
            for node in bfs.iter() {
                match node.borrow().left.as_ref(){
                    Some(x) => row.push(Rc::clone(x)),
                    None => (),
                }
                match node.borrow().right.as_ref(){
                    Some(x) => row.push(Rc::clone(x)),
                    None => (),
                }
            }
            bfs = row;
        }
        ans
    }
}
