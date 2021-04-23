use std::fmt::{Display, Formatter};
use std::ptr::null;
use std::mem::transmute;

pub fn safety_with_references() {
    let mut object = SomeHeavyWeightObject::new();
    object.boxed.field = 289;
    let r1= &object.boxed.field;
    // cannot create this mutable reference, the target for r1 might become invalid
    // by modifying the box to point somewhere else

    // let r2 = &mut object;
    dbg!(r1);
}

pub fn safety_in_arithmetics(input: i8) -> i8 {
    let x = 91i8;
    // let y = 69i8;
    x + input
}

pub fn safety_in_error_handling() -> Result<(), Error> {
    ErrorExample::new()?.errors()?.all()?.the()?.way();

    let _ = match ErrorExample::new()  {
        Ok(new) => /* */ Ok(()),
        Err(e) => Err(e)
    };

    Ok(())
}

pub fn unsafety() -> f32 {
    let mut x: * mut i32 = null::<i32>() as *mut i32;
    unsafe {
        let current = *x;
        *x += 1;
        return transmute(current)
    }
}

pub struct ErrorExample {}
impl ErrorExample {
    fn new() -> Result<ErrorExample, Error> {
        Ok(ErrorExample {})
    }

    fn errors(&self) -> Result<&Self, Error> {
        Err(Error {})
    }

    fn all(&self) -> Result<&Self, Error> {
        Ok(&self)
    }

    fn the(&self) -> Result<&Self, Error> {
        Ok(&self)
    }

    fn way(&self) -> Result<&Self, Error> {
        Ok(&self)
    }


}

#[derive(Debug)]
pub struct Error {}
impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl std::error::Error for Error {}



struct SomeHeavyWeightObject {
    boxed: Box<Boxed>
}
impl SomeHeavyWeightObject {
    fn new() -> SomeHeavyWeightObject {
        SomeHeavyWeightObject{boxed: Box::new(Boxed{field: 30})}
    }
}
struct Boxed {
    field: u32
}