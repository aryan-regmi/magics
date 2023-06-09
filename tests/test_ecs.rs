use magics::prelude::*;

#[derive(Debug, Clone)]
struct Health(u32);
impl Component for Health {
    // fn as_any(&self) -> &dyn Any {
    //     self as &dyn Any
    // }
    //
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
}

#[derive(Debug, Clone)]
struct Age(u32);
impl Component for Age {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    //
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
}

#[derive(Debug, Clone)]
struct Name(&'static str);
impl Component for Name {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    //
    // fn as_any_mut(&mut self) -> &mut dyn Any {
    //     self
    // }
}

fn system1(mut ctx: Context) {
    println!("[System 1] Spawning Entity 0");
    let _entity0 = ctx
        .spawn()
        .with(Age(22))
        .with(Health(100))
        .with(Name("Entity 0"))
        .build();
    println!("{}: Entity 0 spawned", _entity0);

    println!("[System 1] Spawning Entity 2");
    let _entity2 = ctx.spawn().with(Age(18)).with(Name("Entity 2")).build();
    println!("{}: Entity 2 spawned", _entity2);
}

fn system2(mut ctx: Context) {
    println!("[System 2] Spawning Entity 1");
    let _entity1 = ctx.spawn().with(Health(50)).with(Name("Entity 1")).build();
    println!("{}: Entity 1 spawned", _entity1);
}

#[test]
fn can_create_ecs() {
    App::new().add_system(system1).add_system(system2).run();
}
