#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Box<Self> {
        Box::new(ListNode { val, next: None })
    }

    #[allow(dead_code)]
    fn list_traversal(&self) {
        println!("\nVal: {}", self.val);
        let mut pointer: &Option<Box<ListNode>> = &self.next;
        loop {
            if pointer.is_none() {
                println!("-/");
                break;
            } else {
                let valid = pointer.as_ref().unwrap();
                println!("Val: {}", valid.val);
                pointer = &valid.next;
            }
        }
    }

    #[allow(dead_code)]
    pub fn recursive_list_traversal(node: &Self) {
        println!("Val from recursive: {}", node.val);
        match &node.next {
            Some(val) => Self::recursive_list_traversal(val),
            None => println!("End of list"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_list() {
        let mut my_list_node = ListNode::new(10);
        my_list_node.next = Some(ListNode::new(11));
        // my_list_node.list_traversal();
        ListNode::recursive_list_traversal(&my_list_node);
        // dbg!(&my_list_node);
        assert_eq!(my_list_node.val, 10);
    }
}
