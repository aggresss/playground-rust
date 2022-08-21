
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }

    fn reversed(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: &ListNode = &list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {

    }
}