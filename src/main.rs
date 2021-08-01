use futures::future::try_join_all;
use std::fmt;

#[derive(Debug)]
enum MyError {
    Other,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Some other error occured!"),
        }
    }
}

async fn do_some_work(task_name: &str) -> Result<(), MyError> {
    println!("This is task {}, doing some work", task_name);
    let reader = tokio::io::BufReader::new(tokio::io::stdin());
    let buffer = Vec::new();

    let fut = tokio_io::io::read_until(reader, b'\n', buffer).await;
    println!("Input was: {:?}", buffer);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let results = try_join_all(vec![
        do_some_work("Task 1"),
        do_some_work("Task 2"),
        do_some_work("Task 3"),
    ])
    .await;
    results?;

    Ok(())
}
