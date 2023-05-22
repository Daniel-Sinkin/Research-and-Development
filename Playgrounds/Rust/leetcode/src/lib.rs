#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
        next: None,
        val: val,
    }
  }

  pub fn get_last(&mut self) -> &mut ListNode {
    let mut node = self;
    loop {
        match &mut node.next {
            Some(next) => node = next,
            None => break,
        }
    }
    node
  }


  pub fn push(&mut self, val: i32) {
    let mut last = self.get_last();

    last.next = Some(Box::new(ListNode::new(val)));
  }

  pub fn from_list(nums: Vec<i32>) -> Self {
    if nums.len() == 0 {
        panic!("Can't generate a vector from an empty list");
    }

    if nums.len() == 1 {
        return ListNode::new(nums[0].clone());
    }

    // Note that this `head` is the head of the children, not the head of the linked list
    // i.e. on list [2, 4, 3] we have a node (2) -> [(4) -> [(3)]] and head is the node for '4'
    let mut head = Box::new(ListNode::new(nums[1].clone()));
    let mut curr = &mut head;

    for n in nums.iter().skip(2) {
        curr.next = Some(Box::new(ListNode::new(n.clone())));
        curr = curr.next.as_mut().unwrap();
    }

    ListNode {val: nums[0].clone(), next: Some(head)}
  }
}
