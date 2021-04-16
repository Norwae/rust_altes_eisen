mod keywords;
mod ownership;
mod generic;
mod modular {
    // see the cargo.toml file for this
}
mod safety;
mod futures;

fn main() {
    println!("Hello, world!");

    println!("{}", generic::add_three(1,2,3));
    println!("{}", safety::safety_in_arithmetics(85));
    safety::safety_with_references();
     safety::safety_in_error_handling();
}
