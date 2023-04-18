use magics::prelude::*;

#[derive(Debug)]
struct Health(u32);
impl Component for Health {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
struct Age(u32);
impl Component for Age {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
struct Name(&'static str);
impl Component for Name {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn setup(mut ctx: Context) {
    ctx.spawn(EntityBuilder::new().with(Name("Enity 0")));
    ctx.spawn(EntityBuilder::new().with(Age(10)).with(Name("Enity 1")));
    ctx.spawn(
        EntityBuilder::new()
            .with(Health(50))
            .with(Age(22))
            .with(Name("Entity 2")),
    );
}

fn check_ages(mut ctx: Context) {
    // FIX: Remove this when Stages have been added
    std::thread::sleep(std::time::Duration::from_secs(1));

    let age_query = ctx.query(QueryBuilder::new().with::<Age>());

    let mut age_query_iter = age_query.into_iter();
    assert_eq!(age_query_iter.next().unwrap().get::<Age>().unwrap().0, 10);
    assert_eq!(age_query_iter.next().unwrap().get::<Age>().unwrap().0, 22);
    assert!(age_query_iter.next().is_none());
}

fn update_names(mut ctx: Context) {
    // FIX: Remove this when Stages have been added
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut name_query = ctx
        .query(QueryBuilder::new().with_mut::<Name>())
        .into_iter();

    let mut e0_name = name_query.next().unwrap();
    let e0_name = e0_name.get_mut::<Name>().unwrap();
    *e0_name = Name("Updated Enity 0");

    let mut e1_name = name_query.next().unwrap();
    let e1_name = e1_name.get_mut::<Name>().unwrap();
    *e1_name = Name("Updated Enity 1");

    let mut e2_name = name_query.next().unwrap();
    let e2_name = e2_name.get_mut::<Name>().unwrap();
    *e2_name = Name("Updated Enity 2");
}

fn check_updated_names(mut ctx: Context) {
    // FIX: Remove this when Stages have been added
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut name_query = ctx.query(QueryBuilder::new().with::<Name>()).into_iter();

    let mut e0_name = name_query.next().unwrap();
    let e0_name = e0_name.get_mut::<Name>().unwrap();
    assert_eq!(e0_name.0, "Updated Entity 0");

    let mut e1_name = name_query.next().unwrap();
    let e1_name = e1_name.get_mut::<Name>().unwrap();
    assert_eq!(e1_name.0, "Updated Entity 1");

    let mut e2_name = name_query.next().unwrap();
    let e2_name = e2_name.get_mut::<Name>().unwrap();
    assert_eq!(e2_name.0, "Updated Entity 2");
}

#[test]
fn test_queries() {
    App::new()
        .add_system(setup)
        .add_system(check_ages)
        .add_system(update_names)
        .add_system(check_updated_names)
        .run();
}
