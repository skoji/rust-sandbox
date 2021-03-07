use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

struct MyLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl MyLog {
    pub fn new_empty() -> MyLog {
        MyLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(new.clone());
            }
            None => {
                self.head = Some(new.clone());
            }
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(next) => {
                    self.head = Some(next);
                }
                None => {
                    self.tail.take();
                }
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("something wrong")
                .into_inner()
                .value
        })
    }
}

impl Drop for MyLog {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn main() {
    let mut mylog = MyLog::new_empty();
    mylog.append("first".to_string());
    mylog.append("second".to_string());
    mylog.append("third".to_string());
    while let Some(log) = mylog.pop() {
        println!("{}", log);
    }
}
