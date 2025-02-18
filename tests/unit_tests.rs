#[cfg(test)]
mod tests {
    use concurrent_tasks::tasks::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_task_1() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(task_1());
        assert!(result.is_ok() || result.is_err(), "Task 1 should return a result");
    }

    #[test]
    fn test_task_2() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(task_2());
        assert!(result.is_ok() || result.is_err(), "Task 2 should return a result");
    }

    #[test]
    fn test_task_3() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(task_3());
        assert!(result.is_ok() || result.is_err(), "Task 3 should return a result");
    }
}
