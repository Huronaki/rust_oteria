fn main() {
    let hp = 100;
    println!("level 1: {hp} hp");

    let hp = hp * 2;
    println!("level 2: {hp} hp");

    let hp = hp * 2;
    println!("level 3: {hp} hp");

    let mut potions = 0;
    potions += 1;
    potions += 1;
    potions += 1;

    println!("potions: {potions}");
}
