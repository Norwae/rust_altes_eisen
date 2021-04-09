
fn ownership_1(input: String) -> String {
    let mut string1 = String::new();
    string1.push_str("I got ");
    string1.push_str(&input);
    drop(input);
    // explicitly destroy input, by giving it away to the drop function
    let string2 = string1;
    // transfer of ownership
    // the "name" string1 becomes  invalid but the actual object is not destroyed
    // any references to string1 would need to be cleaned up for the move to happen
    let string3 = string2.clone();
    // copy of other object, ownership is not transferred
    // thus, references may continue to exist

    return string3; // ownership of string3 passes to caller
    // string2 falls out of scope, and is destroyed
}

struct Example;

impl Example {
    fn ownership_method(self) {
        // this takes ownership of the self-object, the caller loses it
    }

    fn borrowed_method(&self) {
        // this just borrows the object, and does not alter the owner in any way
    }

    fn mutable_borrowed_method(&mut self) {
        // this borrows the object mutably (more on that later)
    }
}

fn ownership_2() {
    let mut example1 = Example;

    example1.borrowed_method();
    // this is exactly the same as
    (&example1).borrowed_method();

    let ref1 = &example1;
    let ref2 = &example1;

    drop2(ref1, ref2);

    example1.mutable_borrowed_method();
    // this is exactly the same as
    (&mut example1).mutable_borrowed_method();

    let mref1 = &mut example1;
    // let mref2 = &mut example1;
    // cannot create a second mutable reference

    // the ref1 and ref2 used to prevent this, but the language got smarter
    // and recognized that the references are unused and can be discarded early

    example1.ownership_method();
    // as above, mref1 does not block ownership transfer because the compiler
    // can prove it is no longer needed and can be considered already dropped
}

// Advanced territories here

struct  ReferenceContainer<'borrowed, A> {
    contents: &'borrowed A
}

pub fn ownership_3() {
    let x = Example;
    let reference = package_reference(&x, &192);

    // drop(x);

    // cannot move x since the reference must remain valid

    dbg!(reference.contents as *const Example);
}

fn package_reference<'a, 'b, T>(contents: &'a T, unused: &'b u32) -> ReferenceContainer<'a, T> {
    ReferenceContainer {
        contents
    }
}


fn drop2<A>(one: A, two: A) {}

