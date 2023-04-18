use std::sync::Mutex;

use magics::prelude::*;

static MVAL: Mutex<u8> = Mutex::new(0);

fn setup(_ctx: Context) {
    let mut mval = MVAL.lock().unwrap();
    assert_eq!(*mval, 0);

    *mval = 1;
    drop(mval);

    println!("Setup");
}

fn stage1_func1(_ctx: Context) {
    println!("F1");
    assert_eq!(*MVAL.lock().unwrap(), 1);
}

fn stage1_func2(_ctx: Context) {
    println!("F2");
    assert_eq!(*MVAL.lock().unwrap(), 1);
}

fn free_system1(_ctx: Context) {
    println!("Free System 1");
}

fn free_system2(_ctx: Context) {
    println!("Free System 2");
}

#[test]
fn test_stages() {
    // free_system1 & free_system2 run independent of any other system
    //
    // setup always runs before stage1_func(1 & 2)
    //
    // stage1_func1 and stage1_func2 run independent of any other system in their stage

    App::new()
        .add_stage(Stage::run_order(0).with(setup))
        .add_stage(Stage::run_order(1).with(stage1_func1).with(stage1_func2))
        .add_system(free_system1)
        .add_system(free_system2)
        .run();
}
