use futures::future::try_join_all;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum MyError {
    FromUtf8(String),
    IO(String),
    Join(String),
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromUtf8(e) => write!(f, "From UTF8 Error: {}", e),
            Self::IO(e) => write!(f, "IO Error: {}", e),
            Self::Join(e) => write!(f, "Join Error: {}", e),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(error.to_string())
    }
}

impl From<tokio::task::JoinError> for MyError {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Join(error.to_string())
    }
}

async fn do_some_work(task_name: &str, sleep_sec: u8, notify: Arc<Notify>) -> Result<(), MyError> {
    notify.notified().await;
    println!("This is task {}, doing some work", task_name);
    notify.notify_one();

    for i in 0..sleep_sec {
        sleep(Duration::from_secs(1)).await;
        println!("task {} slept for {} seconds", task_name, i);
    }

    notify.notified().await;
    println!("Input for task {}, please?", task_name);
    let result = tokio::task::spawn_blocking(|| -> Result<String, MyError> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    })
    .await??;
    println!("Input for {} was: {:?}", task_name, result);
    notify.notify_one();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let notify = Arc::new(Notify::new());
    let future_vec = vec![
        do_some_work("A", 1, notify.clone()),
        do_some_work("B", 10, notify.clone()),
        do_some_work("C", 20, notify.clone()),
    ];
    notify.notify_one();
    let results = try_join_all(future_vec).await;
    results?;

    Ok(())
}
