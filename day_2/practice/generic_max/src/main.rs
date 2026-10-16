// TODO


fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut max = list.first()?;
    for n in list {
        if n > max {
            max = n;
        }
    }
    Some(max)
}

fn main() {
    let numbers = [3, 41, 5, 2];
    let chars = ['a', 'z', 'm'];
    let fruits = ["pear", "apple"];
    let empty: [i32; 0] = [];

    println!("largest of {:?} -> {:?}", numbers, largest(&numbers));
    println!("largest of {:?} -> {:?}", chars, largest(&chars));
    println!("largest of {:?} -> {:?}", fruits, largest(&fruits));
    println!("largest of {:?} -> {:?}", empty, largest(&empty));
}