use std::mem::size_of;
use std::ops::Add;

struct TwoOf<A> {
    first: A,
    int_in_the_middle: i32,
    second: A
}


// possible layouts in memory:
// int_in_the_middle first second
// int_in_the_middle second first
// first int_in_the_middle second
// ...

type LargeArray = [u8;2048];

pub fn sizing() {
    // generic types are actually different per type parameter
    // invocation, up to and including their memory layout
    dbg!(size_of::<TwoOf<u8>>());
    dbg!(size_of::<TwoOf<u32>>());
    dbg!(size_of::<TwoOf<LargeArray>>());
}

impl <A> TwoOf<A> {
    fn second_param<B>(&self, other: B) {
        // This code is re-generated for every combination of A and B
        // that is used to invoke the method (for example, to drop the other object)

        // note that we cannot "specialize" the method as in C++ for a particular B
    }
}

#[inline(never)]
pub fn add_three<A: Add<Output=A>>(first: A, second: A, third: A) -> A {
    first.add(second).add(third)
}

pub fn dynamic(stringlike: &dyn AsRef<str>) {
    println!("The string is {}", stringlike.as_ref())
}

// the emitted definition is heavily optimized anyway, see machine code

/*
play.rust-lang.org:

use std::ops::Add;

#[inline(never)]
pub fn add_three<A: Add<Output=A>>(first: A, second: A, third: A) -> A {
    first.add(second).add(third)
}

fn main() {
 for i in 0..4 {

    println!("{}", add_three(i,i+1, i-1))
 }
}
 */