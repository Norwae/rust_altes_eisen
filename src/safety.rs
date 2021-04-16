use std::fmt::{Display, Formatter};

fn safety_with_references() {
    let mut object = SomeHeavyWeightObject::new();
    let r1= &object.boxed.field;
    // cannot create this mutable reference, the target for r1 might become invalid
    // by modifying the boxed to point somewhere else

    // let r2 = &mut object;
    dbg!(r1);
}

pub fn safety_with_arithmetics(input: i8) -> i8 {
    let x = 91i8;
    // let y = 69i8;
    x + input
}

fn safety_with_error_handling() -> Result<(), Error> {
    ErrorExample::new().errors?.all?.the?.way()?;

    Ok(())
}

struct ErrorExample {}
impl ErrorExample {
    fn new() -> Result<ErrorExample, Error> {
        Ok(ErrorExample {})
    }

    fn errors(&self) -> Result<&self, Error> {
        Err(Error)
    }

    fn all(&self) -> Result<&self, Error> {
        Ok(&self)
    }

    fn the(&self) -> Result<&self, Error> {
        Ok(&self)
    }

    fn way(&self) -> Result<&self, Error> {
        Ok(&self)
    }


}

#[derive(Debug)]
struct Error {}
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