use std::future::Future;
use std::error::Error;
use std::task::{Context, Poll};
use std::pin::Pin;

async fn foo(x: i32) -> i32 {
    // enables the .await pseudo-method for Future types, as well as
    // modifying the return type to Future<i32>

    let input = do_network_io().await;
    write_to_fs(&input).await.expect("Can't happen, going to panic!");
    x
}

fn foo_desugared(x: i32) -> impl Future<Output=i32> {
    enum State{
        Initial(i32),
        AfterInput(i32, String)
    }

    impl Future for State {
        type Output = i32;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match *self {
                State::Initial(_) => todo!(),
                State::AfterInput(_,_) => todo!()
            }
        }
    }

    State::Initial(x)
}

async fn write_to_fs(input: &str) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn do_network_io() -> impl Future<Output=String> {
    struct Nope {}
    impl Future for Nope {
        type Output = String;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            todo!()
        }
    }
    Nope{}
}