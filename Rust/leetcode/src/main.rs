use leetcode::*;

#[allow(unused_variables)]
fn main() {
  let list1 = vec![2, 4, 3];
  let list2 = vec![5, 6, 4, 7];

  let mut l1 = ListNode::from_list(list1);
  let mut l2 = ListNode::from_list(list2);

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

  let t = &l1.get_last();

  println!("{}", &t.val);

  //  l1.push(-1);
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::ListNode;

    #[test]
    fn listnode_push() {
        let mut l1 = ListNode::from_list(vec![1,2,3,4]);
        l1.push(5);
        let mut l2 = ListNode::from_list(vec![1,2,3,4,5]);
        assert_eq!(&l1, &l2);
    }
}
