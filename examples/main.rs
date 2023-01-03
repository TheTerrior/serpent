use std::{io::{self, stdin}, cell::RefCell};

use serpent::{self};

fn main() {
    //let x = serpent::add(1, 2);
    //println!("{}", x);
    //test_ncurses();
    build_ui();
}

fn test_arrays<const N: usize>() -> [i32; N] {
    let test_arr = [0; N];
    return test_arr;
}

struct test_struct {
    pub first: i32,
    pub second: i32,
}

fn build_ui() {

    let [x, y, z] = test_arrays::<3>();
    let [a, b, c, d] = test_arrays::<4>();

    let str_test = RefCell::new(test_struct{first: 31, second: 30});
    str_test.borrow_mut().first = 29;
    println!("{}", str_test.borrow().first);
    

    /* 

    let mut ui = serpent::from(
        Page::new("Main")
            .keybinds(Keybinds::main())
            .children(vec![

                Page::new("Inner")
                    .keybinds(Keybinds::default()),

            ])
    );
    */

    let mut ui = serpent::new();
    let (page_index, partition) = ui.new_page();
    let partition_size = partition.borrow().size;


    //panic!();
    //serpent::start();

    /* 
    let mut ui = serpent::from(
        Page::new("Main")
            .keybinds(Keybinds::main())
            .root(Text::new("Hello").align(Align::CenterLeft).tag("_TEXT0_").colors(Colors::default()))
            .splith(Text::new("Waddup my man").align())
    );
    */

    //let mut ui = serpent::from(
    //    Page::new("Main")
    //        .keybinds(Keybinds::main())
    //        .elements(vec![
    //            (Split::Horizontal(0.5), Element::new_text("Hello").align(Align::CenterLeft).tag("_TEXT0_").colors(Colors::default())),
    //            (Split::None, Element::new_text("Waddup my man").align(Align::Left)),
    //        ])

    //);







    //ui.show();


    //serpent::stop();

}

//fn test_ncurses() {
//    let ui = serpent::new();
//    serpent::restart();
//    ui.show();
//    serpent::stop();
//    let mut x: String = String::new();
//    stdin().read_line(&mut x);
//    println!("{}", x);
//
//    serpent::restart();
//    ui.show();
//    serpent::stop();
//}
