struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        let mut l = &mut list1;
        let mut r = &mut list2;
        while l.is_some() || r.is_some() {
            match (&l, &r) {
                (None, Some(node)) | (Some(node), None) => match cur {
                    None => {
                        head = Some(node.clone());
                        return head;
                    }
                    Some(h) => {
                        h.next = Some(node.clone());
                        return head;
                    }
                },
                (Some(node1), Some(node2)) => {
                    let (cur_val, ln, rn) = if node1.val < node2.val {
                        (node1.val, node1.next.clone(), Some(node2.to_owned()))
                    } else {
                        ( node2.val, Some(node1.to_owned()), node2.next.clone() )
                    };
                    let n = Some(Box::new(ListNode {
                        val: cur_val,
                        next: None,
                    }));
                    match cur {
                        None => {
                            head = n;
                            cur = &mut head;
                        },
                        Some(c) => {
                            c.next = n;
                            cur = &mut c.next;
                        }
                    };
                }
                (None, None) => {
                    return None;
                }
            }
        }
        head
    }
}
