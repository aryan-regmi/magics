use magics::prelude::*;

#[derive(Debug)]
struct Name(&'static str);
impl Component for Name {}

#[derive(Debug)]
struct Health(u32);
impl Component for Health {}

#[derive(Debug)]
struct Age(u32);
impl Component for Age {}

fn setup(mut ctx: Context) {
    // NPC
    let _npc = ctx.spawn().with(Name("NPC")).with(Health(100));

    // Player
    let _player = ctx
        .spawn()
        .with(Name("Player"))
        .with(Health(75))
        .with(Age(22));
}

fn get_names(mut ctx: Context) {
    let mut names = ctx.query(QueryBuilder::new().with::<Name>());

    assert_eq!(
        names.next().unwrap().get_component::<Name>().unwrap().0,
        "NPC"
    );
    assert_eq!(
        names.next().unwrap().get_component::<Name>().unwrap().0,
        "Player"
    );
}

#[test]
fn can_query_single_component() {
    App::new()
        .add_stage(Stage::run_order(0).with(setup))
        .add_stage(Stage::run_order(1).with(get_names))
        .run();
}
