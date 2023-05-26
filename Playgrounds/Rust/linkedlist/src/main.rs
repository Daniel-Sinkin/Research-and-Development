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

    fn from_list(vec: &[i32]) -> Option<Self> {
        if vec.is_empty() {
            return None;
        }

        if let Some(x) = vec.first() {
            let mut ln = ListNode::new(*x);

            let nxt = ListNode::from_list(&vec[1..]);

            if let Some(y) = nxt {
                ln.next = Some(Box::new(y));
            }

            return Some(ln);
        } else {
            return None;
        }
    }

    fn to_string(&self) -> String {
        let mut s: String = format!("({})", self.val);

        if let Some(x) = &self.next {
            s.push_str(&format!(" -> {}", &x.to_string()));
        }
        s
    }

    fn insert_after(&mut self, val: i32) {
        if let Some(x) = &self.next {
            panic!();
        } else {
            self.next = Some(Box::new(ListNode::new(val)));
        }
    }
}

fn main() {
    let mut ln: ListNode = ListNode::new(0);
    let mut ln_next: ListNode = ListNode::new(1);
    let ln_next_next: ListNode = ListNode::new(2);
    ln_next.next = Some(Box::new(ln_next_next));

    ln.next = Some(Box::new(ln_next));

    let v: Vec<i32> = vec![1, 2, -7, 4];

    dbg!(&v);
    dbg!(&v[1..]);
    dbg!(&v[2..]);
    dbg!(&v[3..]);
    dbg!(&v[4..]);

    let x = ListNode::from_list(&v);

    let mut y = ListNode::from_list(&v[3..]).unwrap();

    if let Some(y) = x {
        println!("{}", y.to_string());
    } else {
        println!("Empty!");
    }

    y.insert_after(-10);

    println!("{}", y.to_string());
}
