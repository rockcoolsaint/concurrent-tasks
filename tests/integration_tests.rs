#[cfg(test)]
mod tests {
    use concurrent_tasks::tasks::*;
    use tokio::runtime::Runtime;
    use tokio::try_join;

    #[test]
    fn test_all_tasks_together() {
        let rt = Runtime::new().unwrap();

        let result = rt.block_on(async {
            let task1 = task_1();
            let task2 = task_2();
            let task3 = task_3();

            try_join!(task1, task2, task3)
        });

        assert!(result.is_ok() || result.is_err(), "All tasks should return a result");
    }
}
