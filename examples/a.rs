extern crate serpent;

fn main() {
    let x = serpent::add(1, 2);
    println!("{}", x);

    let y = serpent::internal_folder::testing::add_one(1);
    println!("{}", y);
}