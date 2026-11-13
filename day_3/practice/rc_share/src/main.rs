use std::rc::Rc;

#[derive(Debug)]
enum List {
    Node(i32, Rc<List>),
    End,
}

fn inspect(list: &List) -> Option<(i32, usize)> {
    match list {
        List::Node(value, tail) => Some((*value, Rc::strong_count(tail))),
        List::End => None,
    }
}

fn main() {
    let a = Rc::new(List::Node(1, Rc::new(List::End)));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = List::Node(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = List::Node(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

        println!("c head = {:?}", inspect(&c));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("b head = {:?}", inspect(&b));
}
