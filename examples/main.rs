use std::io::{self, stdin};

use serpent::{self, internal::*};

fn main() {
    //let x = serpent::add(1, 2);
    //println!("{}", x);
    //test_ncurses();
    build_ui();
}

fn build_ui() {
    serpent::start();

    let mut ui = serpent::from(
        Page::new("Main")
            .keybinds(Keybinds::main())
            .children(vec![

                Page::new("Inner")
                    .keybinds(Keybinds::default()),

            ])
    );

    ui.show();


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
