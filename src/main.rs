mod tasks;
mod errors;
mod shutdown;
mod logging;

use crate::tasks::{task_1, task_2, task_3, network_request, database_operation};
use crate::errors::TaskError;
use crate::shutdown::shutdown_system;
use tokio::try_join;
use log::{error, info};
// use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    logging::init();

    match run_tasks().await {
        Ok(_) => {
            info!("System completed without critical errors.");
            std::process::exit(0); // Success exit code
        }
        Err(e) => {
            error!("System encountered an error: {}", e);
            std::process::exit(1); // Failure exit code
        }
    }
}

async fn run_tasks() -> Result<(), TaskError> {
    let (shutdown_tx, shutdown_rx) = shutdown::shutdown_channel();

    // Spawn shutdown system to listen for shutdown signals
    let shutdown_handle = tokio::spawn(shutdown_system(shutdown_rx));

    // Spawn tasks
    let task1 = tokio::spawn(async { task_1().await });
    let task2 = tokio::spawn(async { task_2().await });
    let task3 = tokio::spawn(async { task_3().await });
    let task4 = tokio::spawn(async { network_request().await });
    let task5 = tokio::spawn(async { database_operation().await });

    // Wait for all tasks to finish
    let result = try_join!(task1, task2, task3, task4, task5);

    match result {
        Ok((res1, res2, res3, res4, res5)) => {
            res1?;
            res2?;
            res3?;
            res4?;
            res5?;
            info!("All tasks completed successfully.");
        }
        Err(e) => {
            error!("A task failed: {:?}", e);
            shutdown_tx.send(()).ok(); // Trigger shutdown signal
            return Err(TaskError::CriticalError("One or more tasks failed. Shutting down.".into()));
        }
    }

    // shutdown_rx.await.ok(); // Wait for shutdown signal

    // Wait for the shutdown listener to complete
    let _ = shutdown_handle.await;

    Ok(())
}
