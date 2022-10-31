extern crate serpent;

enum TestEnum<'a> {
    first(String),
    second(&'a dyn Fn() -> ()),
    third(&'a str),
}
impl<'a> TestEnum<'a> {
    pub fn run_if_func(&self) {
        match self {
            TestEnum::first(_) => {println!("no func");},
            TestEnum::second(func) => func(),
            TestEnum::third(_) => {println!("no func");},
        }
    }
}

enum SecondEnum {
    b(u16),
    functionfirst(fn(u8) -> i32),
}
impl SecondEnum {
    pub fn run_fn(&self) {
        if let SecondEnum::functionfirst(func) = self {
            println!("{}", func(8));
        } else {
            println!("not a function");
        }
    }
}

fn testfunc(inp: u8) -> i32 {
    return inp as i32;
}

fn main() {
    let cl = || {
        println!("hi");
    };
    let x = TestEnum::first(String::from("hello"));
    let y = TestEnum::second(&cl);
    let z = TestEnum::third("hiii");

    x.run_if_func();
    y.run_if_func();
    z.run_if_func();

    let b = || {
        println!("{}", serpent::add(1, 12));
    };
    let a = TestEnum::second(&b);

    a.run_if_func();

    println!("------");

    let c = SecondEnum::functionfirst(testfunc);
    println!("trying to fun function here {}", testfunc(3));
    c.run_fn();


    


}