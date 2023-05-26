use std::fmt;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }

    fn new_vector(vec: Vec<i32>) -> Option<Self> {
        Some(ListNode {
            val: vec[0],
            next: None,
        })
    }

    fn to_string(&self) -> String {
        let mut s: String = format!("({})", self.val);

        if let Some(x) = &self.next {
            s.push_str(&format!(" -> {}", &x.to_string()));
        }
        s
    }
}

fn main() {
    let mut ln: ListNode = ListNode::new(0);
    let mut ln_next: ListNode = ListNode::new(1);
    let ln_next_next: ListNode = ListNode::new(2);
    ln_next.next = Some(Box::new(ln_next_next));
    ln.next = Some(Box::new(ln_next));

    println!("{}", ln.to_string());
}
