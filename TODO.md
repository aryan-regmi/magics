# App Scheduler

- Create a thread for each core
  - Find number of cores:
      ```rust
            let _num_cores: usize = std::thread::available_parallelism()
                .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
                .into();
      ```

- Split the systems array evenly into each thread

- Run through the queue of each thread and run the system

