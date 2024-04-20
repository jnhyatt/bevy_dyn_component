use std::marker::PhantomData;

use bevy::{ecs::component::ComponentId, prelude::*};
use bevy_dyn_component::prelude::*;

fn main() {
    App::new()
        .add_plugins(dynamic_components_plugin)
        .add_systems(Main, relations_at_home)
        .run();
}

fn relations_at_home(world: &mut World) {
    // Mom, can we have relations?
    // Mom: we have relations at home
    // Relations at home:

    let apples = world.spawn(()).id();
    init_relation::<Eats>(world, apples); // Allows apples to be used for the Eats relation

    // Now our apples have a marker component for each "relation" they can be used for. For example,
    // for the "Eats" relation, apples has a RelationMarker<Eats>(some_component). We can pull this
    // component off the entity and attach it to anyone who we want to eat apples:
    let &RelationMarker(eats_apples, _) =
        world.entity(apples).get::<RelationMarker<Eats>>().unwrap();
    let bob = world.spawn(()).insert_dynamic(eats_apples, Eats).id();

    // We can also use that marker to query for anyone that eats apples:
    let mut eats_apples_query = QueryBuilder::<Entity>::new(world)
        .with_id(eats_apples)
        .build();
    let mut everyone_that_eats_apples = eats_apples_query.iter(world);
    assert_eq!(everyone_that_eats_apples.next(), Some(bob));
    assert_eq!(everyone_that_eats_apples.next(), None);
}

#[derive(Component)]
struct Eats;

fn init_relation<R: Component>(world: &mut World, e: Entity) -> ComponentId {
    let related_to = world.dynamic_component::<R>();
    world
        .entity_mut(e)
        .insert(RelationMarker::<R>(related_to, PhantomData));
    related_to
}

#[derive(Component)]
struct RelationMarker<R: Component>(ComponentId, PhantomData<R>);
