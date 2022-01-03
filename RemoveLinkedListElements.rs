// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = None;
        // tail is a mutable reference to dummy
        let mut tail = &mut dummy;

        // take ownership of head
        while let Some(mut node) = head.take() {
            // let head be the next node and also take ownership
            head = node.next.take();
            if node.val != val {
                // let tail value be equal to the node. In the first iteration, dummy becomes head.
                *tail = Some(node);
                // let new tail be the reference to the next node
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        dummy
    }
}
