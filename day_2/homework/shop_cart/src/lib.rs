pub trait Item {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}

pub struct Book {
    pub name: String,
    pub price: f64,
}

impl Item for Book {
    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }
}

pub struct Pen {
    pub name: String,
    pub price: f64,
}

impl Item for Pen {
    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }
}

pub struct Cable {
    pub name: String,
    pub price: f64,
}

impl Item for Cable {
    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }
}

pub struct Cart<T: Item> {
    items: Vec<T>,
}

impl<T: Item> Cart<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn total(&self) -> f64 {
        self.items.iter().map(|item| item.price()).sum()
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }
}

impl<T: Item> Default for Cart<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cart_has_zero_total() {
        let cart = Cart::<Book>::new();

        assert_eq!(cart.count(), 0);
        assert_eq!(cart.total(), 0.0);
    }

    #[test]
    fn cart_counts_items() {
        let mut cart = Cart::<Pen>::new();

        cart.add(Pen {
            name: String::from("red pen"),
            price: 1.25,
        });
        cart.add(Pen {
            name: String::from("blue pen"),
            price: 1.25,
        });

        assert_eq!(cart.count(), 2);
    }

    #[test]
    fn cart_sums_prices() {
        let mut cart = Cart::<Book>::new();

        cart.add(Book {
            name: String::from("Book A"),
            price: 10.0,
        });
        cart.add(Book {
            name: String::from("Book B"),
            price: 15.0,
        });

        assert_eq!(cart.total(), 25.0);
    }
}
