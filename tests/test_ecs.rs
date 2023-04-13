use std::time::Duration;

use magics::prelude::*;

fn system1(_ctx: Context) {
    std::thread::sleep(Duration::from_nanos(1));
    println!("Hello from System1");
}

fn system2(_ctx: Context) {
    std::thread::sleep(Duration::from_nanos(1));
    println!("Hello from System2");
}

#[test]
fn can_create_ecs() {
    App::new().add_system(system1).add_system(system2).run();
}
