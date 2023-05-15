use core::num;
use std::thread::panicking;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
        next: None,
        val: val,
    }
  }

  fn get_last(mut self: ListNode) -> ListNode {
    let mut node = Box::new(self);
      while let Some(next) = node.next {
          node = next;
      }
      *node
  }

  fn push(mut self: ListNode, val: i32) {
    let mut last = self.get_last();

    last.next = Some(Box::new(ListNode::new(val)));
  }

/*
  fn push(self, val: i32) {
    let mut cur = self.next;

    while let Some(y) = cur {
      println!("{}", y.val);
      if let None = &y.next {
        break;
      } else {
        cur = y.next;
      }
    }

    let pusher = ListNode::new(val);
    cur.unwrap().next = Some(Box::new(pusher));
  }
*/

  fn from_list(nums: Vec<i32>) -> Self {
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

#[allow(unused_variables)]
fn main() {
  let list1 = vec![2, 4, 3];
  let list2 = vec![5, 6, 4, 7];

  let l1 = ListNode::from_list(list1);
  let l2 = ListNode::from_list(list2);

  let mut cur1 = Some(&l1);
  let mut cur2 = Some(&l2);

  let mut num1: i32;
  let mut num2: i32;
  
  let mut num_curr: i32;
  let mut carry: bool = false;

  let mut ret_list: Option<ListNode> = None;

  let mut str: String = String::new();

  while let Some(node1) = cur1 {
    num1 = node1.val;
    cur1 = node1.next.as_deref();
    if let Some(node2) = cur2 {
      num2 = node2.val.clone();
      cur2 = node2.next.as_deref();
    } else {
      num2 = 0;
    }

    if carry { num_curr = 1; }
    else { num_curr = 0; }

    num_curr += num1 + num2;

    if num_curr >= 10 {
      num_curr = num_curr % 10;
      carry = true;
    }

    if let None = ret_list {
      ret_list = Some(ListNode::new(num_curr));
    } else if let Some(ref y) = ret_list {
      
    }

    str.push_str("(");
    str.push_str(&i32::to_string(&num_curr));
    str.push_str(") -> "); 

    // str.append"({}) -> (* + {})", num_curr, if carry {1} else {0});
  }

  while let Some(node2) = cur2 {
    num2 = node2.val;
    cur2 = node2.next.as_deref();

    str.push_str("(");
    str.push_str(&i32::to_string(&num2));
    str.push_str(") -> ");
  }

  println!("{}", str);

  println!("\nNEWLINE\n");

  //l1.push(-1);

  println!("{}", &l1.get_last().val);

  // l1.push(-1);
}