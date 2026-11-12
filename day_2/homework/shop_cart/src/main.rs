use shop_cart::{Book, Cable, Cart, Item, Pen};
use std::fs;

fn main() {
    let content = fs::read_to_string("shop_cart.csv").expect("Cannot read file");

    let mut books = Cart::<Book>::new();
    let mut pens = Cart::<Pen>::new();
    let mut cables = Cart::<Cable>::new();

    for line in content.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();

        let category = fields[0];
        let name = fields[1].to_string();
        let price: f64 = fields[2].parse().expect("Invalid price");

        match category {
            "book" => books.add(Book { name, price }),
            "pen" => pens.add(Pen { name, price }),
            "cable" => cables.add(Cable { name, price }),
            _ => println!("Unknown category: {category}"),
        }
    }

    print_cart("Books", &books);
    print_cart("Pens", &pens);
    print_cart("Cables", &cables);

    println!(
        "\nEverything: {:.2}",
        books.total() + pens.total() + cables.total()
    );
}

fn print_cart<T: Item>(title: &str, cart: &Cart<T>) {
    println!(
        "\n{}: {} items, total: {:.2}",
        title,
        cart.count(),
        cart.total()
    );

    for item in cart.items() {
        println!("- {} ({:.2})", item.name(), item.price());
    }
}
