#[cfg(test)]
mod tests {
    use tokio::time::{sleep, Duration};
    use concurrent_tasks::errors::TaskError;

    // A mock failing task
    async fn failing_task() -> Result<(), TaskError> {
        sleep(Duration::from_millis(500)).await;
        Err(TaskError::CriticalError("Mock task failed!".to_string()))
    }

    #[tokio::test]
    async fn test_mock_failure() {
        let result = failing_task().await;
        assert!(result.is_err(), "Mock task should fail");
    }
}
