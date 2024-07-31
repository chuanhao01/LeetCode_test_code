struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // None edge case
        if list1.is_none(){
            return list2;
        }
        if list2.is_none(){
            return list1;
        }
        let mut head: Option<Box<ListNode>> = None ;
        let mut l_cur = list1;
        let mut r_cur = list2;
        while !l_cur.is_none() || !r_cur.is_none(){

        }
    }
}
