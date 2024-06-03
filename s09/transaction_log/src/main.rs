use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: SingleLink
}

#[derive(Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64
}

impl Node {
       // A nice and short way of creating a new node
       fn new(value: String) -> Rc<RefCell<Node>> {
           Rc::new(RefCell::new(Node {
              value: value,
              next: None,
            }))
       }
}

impl TransactionLog {
    // TODO: Implement Display trait to print log with println!()

    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
       let new = Node::new(value);
       match self.tail.take() {
           Some(old) => old.borrow_mut().next = Some(new.clone()),
           None => self.head = Some(new.clone())
       };
       self.length += 1;
       self.tail = Some(new);
   }

    pub fn pop(&mut self) -> Option<String> {
       self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;

            Rc::try_unwrap(head)
            .ok()
            .expect("Something is terribly wrong")
            .into_inner()
            .value
       })
    }
}


fn main() {
    println!("Transaction Log");
    let mut log = TransactionLog::new_empty();
    log.append("Test".to_string());
    log.append("Test2".to_string());
    println!("{:?}", log);
}
