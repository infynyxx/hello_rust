pub struct Node {
    value: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn print_value(&self) {
        println!("Value: {}", self.value);
        match self.next {
            Some(ref node) => {
                node.print_value();
            },
            None => {
                println!("Next node: None");
            }
        }
    }
}

pub fn recursive_data_structure() {    
    let node1 = Node {value: 1, next: None};
    let node2 = Node {value: 2, next: Some(Box::new(node1))};
    let node3 = Node {value: 3, next: Some(Box::new(node2))};
    node3.print_value();
}
