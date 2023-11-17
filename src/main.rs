#[derive(Clone, Debug)]
enum Address {
    Address(Box<Node>),
    Nil,
}

#[derive(Clone, Debug)]
enum NodeValue {
    Value(i32),
    Head,
    Nil,
}

impl NodeValue {
    fn to_string(&self) -> String {
        match self {
            NodeValue::Value(val) => { format!("{}", val)},
            _ => {
                String::from("")
            }
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            NodeValue::Value(val) => val.clone(),
            _ => -1
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    value: NodeValue,
    next: Address,
}

impl Node {
    fn new() -> Node {
        Node {
            value: NodeValue::Head,
            next: Address::Nil,
        }
    }

    fn append(&mut self, value: i32) {
        match self.next {
            Address::Address(ref mut next) => {
                next.append(value);
            }
            Address::Nil => {
                let node = Node {
                    value: NodeValue::Value(value),
                    next: Address::Nil
                };
                self.next = Address::Address(Box::new(node));
            }
        }
    }

    fn to_string(&self) -> String {
        let mut stringified = String::from(format!("{}", self.value.to_string()));
        match self.next {
            Address::Address(ref next) => {
                stringified.push_str(&format!("{}{}",
                    if let NodeValue::Head = self.value { "" } else { ", " }, &next.to_string()));
            }
            Address::Nil => ()
        }
        stringified
    }

    fn get_recursive(&self, target: usize, cur: usize) -> Result<i32, String> {
        if target == cur {
            return Ok(self.value.to_i32())
        }
        match self.next {
            Address::Address(ref next) => {
                next.get_recursive(target, cur + 1)
            }
            Address::Nil => {
                return Err(format!("Index {} not found", target-1))
            }
        }
    }

    fn get(&self, index: usize) -> Result<i32, String> {
        self.get_recursive(index + 1, 0)
    }

    fn remove_recursive(&mut self, target: usize, cur: usize) -> Result<i32, String> {
        if target - 1 == cur {
            return match self.next {
                Address::Address(ref next) => {
                    let val = match self.next {
                        Address::Address(ref next) => {
                            next.value.to_i32()
                        }
                        Address::Nil => {
                            return Err(format!("Index {} not found", target-1))
                        }
                    };
                    self.next = next.next.to_owned();
                    Ok(val)
                }
                Address::Nil => Err(format!("Index {} not found", target-1))

            }
        }
        match self.next {
            Address::Address(ref mut next) => {
                next.remove_recursive(target, cur + 1)
            }
            Address::Nil => Err(format!("Index {} not found", target-1))
        }
    }

    fn remove(&mut self, index: usize) -> Result<i32, String> {
        self.remove_recursive(index + 1, 0)
    }


}

fn main() {
    let mut n = Node::new();
    for i in 0..10 {
        n.append(i);
    }
    println!("{}", n.to_string());
    println!("{}", n.get(9).expect("couldn't find"));
}
