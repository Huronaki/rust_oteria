#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    End,
}

fn walk(list: &List) -> (i32, i32) {
    match list {
        List::End => (0, 0),

        List::Node(value, next) => {
            let (count, sum) = walk(next);

            (count + 1, sum + *value)
        }
    }
}

fn main() {
    let list = List::Node(
        1,
        Box::new(List::Node(2, Box::new(List::Node(3, Box::new(List::End))))),
    );

    let (length, sum) = walk(&list);

    println!("list: {:?}", list);
    println!("length: {}", length);
    println!("sum: {}", sum);
}
