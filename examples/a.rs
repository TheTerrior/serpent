extern crate serpent;
use std::io::{self, stdin};

use serpent::internal::*;

fn main() {
    //let x = serpent::add(1, 2);
    //println!("{}", x);
    //test_ncurses();
    build_ui();
}

fn build_ui() {
    serpent::start();
    let mut ui = serpent::from(
        serpent::new_page("Main Page")
            .text("Hello, World", Align::Center, None)
    );


    serpent::stop();

}

fn test_ncurses() {
    let ui = serpent::new();
    serpent::restart();
    ui.show();
    serpent::stop();
    let mut x: String = String::new();
    stdin().read_line(&mut x);
    println!("{}", x);

    serpent::restart();
    ui.show();
    serpent::stop();
}
