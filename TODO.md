# Separate structs into modules

# Add queries

```rust
struct Query<'q> {
  entities: Vec<Entity<'q>>
}
```


# Add Stages

Stages will add determinism to the systems (helps specify a run order for systems)

- Stages will be named i.e. `Stage::new("setup", system1, system2,...)`

- Order will be specified through `before()` and `after()` functions:
  ```rust 
  App::new()
    .add_stage(Stage::new("setup", system1, system2)) // Only one stage w/out order can be listed: it will run first
    .add_stage(Stage::new("update", system3, system4).after("setup"))
    .run();
  
  ```


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


