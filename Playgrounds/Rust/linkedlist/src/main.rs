#[derive(Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn from_vec(vec: &Vec<i32>) -> Option<Self> {
        ListNode::from_list(&vec)
    }

    fn from_list(list: &[i32]) -> Option<Self> {
        if list.is_empty() {
            return None;
        }

        if let Some(x) = list.first() {
            let mut ln = ListNode::new(*x);

            let nxt = ListNode::from_list(&list[1..]);

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

    fn insert_value(&mut self, val: i32) {
        if let Some(x) = &self.next {
            let ln = ListNode {
                val,
                next: Some(x.clone()),
            };
            self.next = Some(Box::new(ln));
        } else {
            self.next = Some(Box::new(ListNode::new(val)));
        }
    }
}

fn main() {
    let l1_vec = vec![2, 4, 3];
    let l2_vec = vec![5, 6, 6, 2, 5];

    // 1 <= l1.len <= 100 so we can't have empty lists
    let mut l1 = ListNode::from_vec(&l1_vec).unwrap();
    let mut l2 = ListNode::from_vec(&l2_vec).unwrap();

    let mut curr = l1.val + l2.val;
    let mut overflow = false;

    if curr >= 10 {
        curr = curr % 10;
        overflow = true;
    }

    dbg!(curr);

    while let Some(x) = l1.next {
        curr = x.val;
        l1 = *x;

        if let Some(y) = l2.next {
            curr += y.val;
            l2 = *y;
        } else {
            println!("End of l2");
        }

        if overflow {
            curr += 1;
            overflow = false;
        }

        if curr >= 10 {
            // Note that 0 <= x.val, y.val <= 9 so
            // 0 <= curr <= 18 as such curr / 10 == curr % 10
            curr = curr % 10;
            overflow = true;
        }
        dbg!(curr);
    }

    while let Some(y) = l2.next {
        curr = y.val;
        l2 = *y;
        if overflow {
            curr += 1;
            overflow = false;
        }

        if curr >= 10 {
            curr = curr % 10;
            overflow = true;
        }
        dbg!(curr);
    }

    if overflow {
        dbg!(1);
    }
}
