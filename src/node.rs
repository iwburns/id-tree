pub type NodeId = usize;

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data: data,
            parent: None,
            children: Vec::new()
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _node = Node::new(5);
    }

    #[test]
    fn test_data() {
        let five = 5;
        let node = Node::new(five);
        assert_eq!(node.data(), &five);
    }

    #[test]
    fn test_data_mut() {
        let mut five = 5;
        let mut node = Node::new(five);
        assert_eq!(node.data_mut(), &mut five);
    }
}
