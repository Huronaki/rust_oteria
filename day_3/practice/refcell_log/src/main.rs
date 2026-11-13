use std::cell::RefCell;
use std::rc::Rc;

fn append(log: &Rc<RefCell<Vec<String>>>, message: &str) {
    log.borrow_mut().push(message.to_string());
}

fn main() {
    let log_a = Rc::new(RefCell::new(Vec::new()));
    let log_b = Rc::clone(&log_a);
    let log_c = Rc::clone(&log_a);

    append(&log_a, "a: hello");
    append(&log_b, "b: hi back");
    append(&log_c, "a: bye");

    let entries = log_a.borrow();

    println!("shared log ({} entries):", entries.len());

    for entry in entries.iter() {
        println!("- {}", entry);
    }
}
