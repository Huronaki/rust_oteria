use shop_cart::{Book, Cart, Pen};

fn main() {
    let mut books = Cart::<Book>::new();

    books.add(Book {
        name: String::from("Clean Code"),
        price: 29.99,
    });
    books.add(Book {
        name: String::from("The Rust Book"),
        price: 25.15,
    });

    let mut pens = Cart::<Pen>::new();

    pens.add(Pen {
        name: String::from("red pen"),
        price: 1.10,
    });
    pens.add(Pen {
        name: String::from("blue pen"),
        price: 1.25,
    });

    println!(
        "books: {} items, total: {:.2}",
        books.count(),
        books.total()
    );
    println!("- Clean Code");
    println!("- The Rust Book");

    println!("pens: {} items, total: {:.2}", pens.count(), pens.total());

    println!("- red pen");
    println!("- blue pen");

    println!(
        "everything: {:.2} + {:.2} = {:.2}",
        books.total(),
        pens.total(),
        books.total() + pens.total()
    );
}
