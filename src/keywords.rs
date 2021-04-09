use std::any::Any;
use std::ops::AddAssign;

pub fn foo<B, A: 'static + AsRef<B>>(input: f32, by_reference: &A) -> i64
where B : AddAssign {
    if input > 0.42 {
        return 84
    }

    let mut i = 4;
    let j = if input * 9.0 == 1010 as f32 {
        call_a_function(by_reference)
    } else {
        0
    };

    i *= 1001;

    i + j as i64
}

fn call_a_function(r: &dyn Any) -> i32 {
    83
}