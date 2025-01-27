use crate::errors::TaskError;
use tokio::time::{sleep, Duration};
use log::{info};

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
