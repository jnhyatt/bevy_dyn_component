use bevy::prelude::*;
use bevy_dyn_component::prelude::*;

fn main() {
    App::new()
        .add_plugins(dynamic_components_plugin)
        .add_systems(Main, hello_world)
        .run();
}

fn hello_world(world: &mut World) {
    // We can create new dynamic components "templated" on an existing component using
    // `dynamic_component`:
    let marker0 = world.dynamic_component::<Template>();
    // "Templated on" means that these new components will share a data layout, destructor and
    // storage type with the given generic template.

    // To insert an instance of this new component onto an entity, we have to supply the component
    // id and an object of the same type we used to initialize the component to fill out its memory:
    world.spawn(()).insert_dynamic(marker0, Template);

    // We can also create dynamic components using a template that is non-zero-sized:
    let marker1 = world.dynamic_component::<DataTemplate>();
    world
        .spawn(())
        .insert_dynamic(marker1, DataTemplate("Arbitrary data".into()));

    // We can now use the component ids in the query builder API:
    let mut query = QueryBuilder::<()>::new(world).with_id(marker0).build();
    let marker0_count = query.iter(world).count();
    println!("{marker0_count} entities have the marker0 component");

    // Accessing the data is harder. The query builder API doesn't *yet* have a way to add a dynamic
    // component into the query data, and even with `EntityMut`, there's no safe way to grab the
    // data. I'll update this example when the corresponding API has been added.
}

#[derive(Component)]
struct Template;

#[derive(Component)]
struct DataTemplate(String);
