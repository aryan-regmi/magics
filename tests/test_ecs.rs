use magics::prelude::*;

struct Health(u32);
impl Component for Health {}

struct Age(u32);
impl Component for Age {}

struct Name(&'static str);
impl Component for Name {}

fn system1(mut ctx: Context) {
    println!("[System 1] Spawning Entity 0");
    let _entity0 = ctx.spawn(
        EntityBuilder::new()
            .with(Age(22))
            .with(Health(100))
            .with(Name("Entity 0")),
    );
    println!("{}: Entity 0 spawned", _entity0);

    println!("[System 1] Spawning Entity 2");
    let _entity2 = ctx.spawn(EntityBuilder::new().with(Age(18)).with(Name("Entity 2")));
    println!("{}: Entity 2 spawned", _entity2);
}

fn system2(mut ctx: Context) {
    println!("[System 2] Spawning Entity 1");
    let _entity1 = ctx.spawn(EntityBuilder::new().with(Health(50)).with(Name("Entity 1")));
    println!("{}: Entity 1 spawned", _entity1);
}

#[test]
fn can_create_ecs() {
    App::new().add_system(system1).add_system(system2).run();
}
