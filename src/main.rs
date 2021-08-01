use futures::future::try_join_all;
use std::fmt;
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
use tokio::sync::Notify;

#[derive(Debug)]
enum MyError {
    IO(String),
    Other,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO Error: {}", e),
            Self::Other => write!(f, "Some other error occured!"),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.to_string())
    }
}

async fn do_some_work(task_name: &str, notify: Arc<Notify>) -> Result<(), MyError> {
    notify.notified().await;
    println!("This is task {}, doing some work", task_name);
    notify.notify_one();
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buffer = Vec::new();

    notify.notified().await;
    println!("Input for task {}, please?", task_name);
    reader.read_until(b'\n', &mut buffer).await?;
    println!("Input was: {:?}", String::from_utf8(buffer));
    notify.notify_one();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let notify = Arc::new(Notify::new());
    let future_vec = vec![
        do_some_work("Task 1", notify.clone()),
        do_some_work("Task 2", notify.clone()),
        do_some_work("Task 3", notify.clone()),
    ];
    notify.notify_one();
    let results = try_join_all(future_vec).await;
    results?;

    Ok(())
}
