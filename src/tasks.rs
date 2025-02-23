use crate::errors::TaskError;
use tokio::time::{sleep, Duration};
use log::{error, info};

pub async fn task_1() -> Result<(), TaskError> {
    sleep(Duration::from_secs(2)).await; // Simulate work
    if rand::random::<u8>() % 2 == 0 {
        Err(TaskError::CriticalError("Task 1 failed critically".to_string()))
    } else {
        info!("Task 1 completed successfully.");
        Ok(())
    }
}

pub async fn task_2() -> Result<(), TaskError> {
    sleep(Duration::from_secs(1)).await; // Simulate work
    if rand::random::<u8>() % 2 == 0 {
        Err(TaskError::GeneralError("Task 2 failed".to_string()))
    } else {
        info!("Task 2 completed successfully.");
        Ok(())
    }
}

pub async fn task_3() -> Result<(), TaskError> {
    sleep(Duration::from_secs(3)).await; // Simulate work
    if rand::random::<u8>() % 2 == 0 {
        Err(TaskError::CriticalError("Task 3 failed critically".to_string()))
    } else {
        info!("Task 3 completed successfully.");
        Ok(())
    }
}

// Simulate a network request
pub async fn network_request() -> Result<(), TaskError> {
    let delay = rand::random::<u8>() % 5; // Random delay between 0-4 sec
    sleep(Duration::from_secs(delay as u64)).await;
    
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    match reqwest::get(url).await {
        Ok(resp) => {
            info!("Network request successful: {:?}", resp.status());
            Ok(())
        }
        Err(_) => {
            error!("Network request failed!");
            Err(TaskError::GeneralError("Failed to fetch data".into()))
        }
    }
}