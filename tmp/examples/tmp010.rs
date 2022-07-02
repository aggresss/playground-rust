pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messnger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messnger: messenger,
            value: 0,
            max: max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messnger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messnger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messnger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

use std::cell::RefCell;

fn main() {
    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));
        }
    }

}
