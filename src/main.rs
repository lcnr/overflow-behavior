use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
enum NodeMode {
    Current,
    V2,
}

struct Node<'a> {
    mode: NodeMode,
    available_depth: u64,
    did_overflow: Cell<u64>,
    parent: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn root(mode: NodeMode, available_depth: u64) -> Self {
        Node {
            mode,
            available_depth,
            did_overflow: Cell::new(0),
            parent: None,
        }
    }

    fn set_overflow(&self) {
        let mut curr = self;
        loop {
            let prev = curr.did_overflow.get();
            curr.did_overflow.set(prev + 1);
            if prev != 0 { 
                break;
            } else if let Some(parent) = curr.parent {
                curr = parent;
            } else {
                break;
            }
        }
    }

    fn try_spawn_child<'b>(&'b self, counter: &mut Counter) -> Option<Node<'b>> {
        if self.available_depth == 0 {
            self.set_overflow();
            None
        } else {
            counter.count += 1;
            let available_depth = match self.mode {
                NodeMode::Current => {
                    if self.did_overflow.get() > 0 {
                        self.available_depth / 4
                    } else {
                        self.available_depth - 1
                    }
                }
                NodeMode::V2 => {
                    if self.did_overflow.get() > 0 {
                        self.available_depth / 4u64.saturating_pow(self.did_overflow.get() as u32)
                    } else {
                        self.available_depth - 1
                    }
                }
            };
            Some(Node {
                mode: self.mode,
                available_depth,
                did_overflow: Cell::new(0),
                parent: Some(&self),
            })
        }
    }
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    } 

    fn pow_n_tree(&mut self, mode: NodeMode, n: u64, available_depth: u64) -> usize {
        self.pow_n_tree_recur(n, Node::root(mode, available_depth));
        self.count
    }

    fn pow_n_tree_recur(&mut self, n: u64, node: Node<'_>) {
        for i in 0..n {
            if let Some(lhs) = node.try_spawn_child(self) {
                self.pow_n_tree_recur(n, lhs);
            }
        }
    }
}

fn main() {
    let mut prev = 0; 
    for i in 0..512 {
        let new = Counter::new().pow_n_tree(NodeMode::Current, 3, i);
        println!("{i:4}, {new:10}: total-difference {:9}, relative-difference {:3.5}", new - prev, new as f64 / prev as f64);
        prev = new;
    }

    let mut prev = 0; 
    for i in 0..512 {
        let new = Counter::new().pow_n_tree(NodeMode::V2, 3, i);
        println!("{i:4}, {new:10}: total-difference {:9}, relative-difference {:3.5}", new - prev, new as f64 / prev as f64);
        prev = new;
    }
}
