enum Light {
    Red,
    Yellow,
    Green,
}

fn next(light: Light) -> Light {
    match light {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}

fn name(light: &Light) -> &'static str {
    match light {
        Light::Red => "red",
        Light::Green => "green",
        Light::Yellow => "yellow",
    }
}

fn duration(light: &Light) -> u32 {
    match light {
        Light::Red => 30,
        Light::Green => 45,
        Light::Yellow => 5,
    }
}

fn main() {
    let lights = [Light::Red, Light::Green, Light::Yellow];

    let third = lights.get(2).unwrap();
    println!("third light: {}", name(third));

    match lights.get(99) {
        Some(light) => println!("light 99: {}", name(light)),
        None => println!("light 99: none"),
    }

    let red = Light::Red;
    println!("{} lasts {}s", name(&red), duration(&red));

    let after_red = next(red);
    println!("after red: {}", name(&after_red));
    println!("it lasts {}s", duration(&after_red));
}
