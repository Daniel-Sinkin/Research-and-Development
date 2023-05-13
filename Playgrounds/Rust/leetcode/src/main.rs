use core::num;
use std::thread::panicking;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
  pub val: T,
  pub next: Option<Box<ListNode<T>>>
}

impl<T: Clone> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
      ListNode::<T> {
              next: None,
              val
          }
    }

  fn from_list(nums: Vec<T>) -> Self {
    if nums.len() == 0 {
      panic!("Can't generate a vector from an empty list");
    }

    if nums.len() == 1 {
      return ListNode::new(nums[0].clone());
    }

    let mut head = Box::new(ListNode::new(nums[1].clone()));
    let mut curr = &mut head;

    for n in nums.iter().skip(2) {
      curr.next = Some(Box::new(ListNode::new(n.clone())));
      curr = curr.next.as_mut().unwrap();
    }

    ListNode {val: nums[0].clone(), next: Some(head)}
  }
}

fn main() {
  let list1 = vec![2, 4, 3];
  let list2 = vec![5, 6, 4];

  let l1 = ListNode::<i32>::from_list(list1);
  let l2 = ListNode::<i32>::from_list(list2);

  let mut left = l1.next;
  while let Some(x) = left {
    dbg!(x);
  }

}