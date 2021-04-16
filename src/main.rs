mod keywords;
mod ownership;
mod generic;
mod modular {
    // see the cargo.toml file for this
}
mod safety;

fn main() {
    println!("Hello, world!");

    println!("{}", generic::add_three(1,2,3));
    println!("{}", safety::safety_with_arithmetics(85));
}
