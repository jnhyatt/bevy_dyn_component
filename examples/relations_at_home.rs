use bevy::prelude::*;
use bevy_dyn_component::prelude::*;
use relations_at_home::*;

fn main() {
    App::new()
        .add_plugins(dynamic_components_plugin)
        .add_systems(Main, relations_at_home)
        .run();
}

// Mom, can we have relations?
// Mom: we have relations at home
// Relations at home:
fn relations_at_home(world: &mut World) {
    // First we spawn in apples
    let mut apples = world.spawn(());
    // Then we make apples edible, which gives us a component to put on things we want to eat apples
    let (apples, eats_apples) = apples.make_relatable::<Eats>();
    let apples = apples.id();

    // Relate `bob` by `Eats` to `apples` (meaning make Bob eat apples)
    let bob = world.spawn(()).relate_default::<Eats>(apples).id();

    // We can also use that marker to query for anyone that eats apples:
    let mut eats_apples_query = QueryBuilder::<Entity>::new(world)
        .with_id(eats_apples)
        .build();
    let mut everyone_that_eats_apples = eats_apples_query.iter(world);
    assert_eq!(everyone_that_eats_apples.next(), Some(bob));
    assert_eq!(everyone_that_eats_apples.next(), None);

    // If we despawn apples, all instances of `eats_apples` will be removed from the world
    world.entity_mut(apples).despawn();
    let mut everyone_that_eats_apples = eats_apples_query.iter(world);
    assert_eq!(everyone_that_eats_apples.next(), None);
}

#[derive(Component, Default)]
struct Eats;

mod relations_at_home {
    use std::marker::PhantomData;

    use bevy::{ecs::component::ComponentId, prelude::*};
    use bevy_dyn_component::prelude::*;

    #[derive(Component, Deref)]
    struct RelationMarker<R: Component>(#[deref] ComponentId, PhantomData<R>);

    pub trait EntityWorldMutExt {
        fn make_relatable<R: Component>(&mut self) -> (&mut Self, ComponentId);
        fn relate<R: Component>(&mut self, e: Entity, data: R) -> &mut Self;

        fn relate_default<R: Component + Default>(&mut self, e: Entity) -> &mut Self {
            self.relate(e, R::default())
        }
    }

    impl EntityWorldMutExt for EntityWorldMut<'_> {
        /// This inserts a `RelationMarker<R>(x)` onto `self`, marking it as a potential target of
        /// the `R` relation. `x` is the id of a component that represents "related by `R` to this
        /// entity".
        fn make_relatable<R: Component>(&mut self) -> (&mut Self, ComponentId) {
            let related_to = unsafe { self.world_mut() }.dynamic_component::<R>();

            let mut query_state = QueryBuilder::<Entity, ()>::new(unsafe { self.world_mut() })
                .with_id(related_to)
                .build();
            self.observe(
                move |_: Trigger<OnRemove, RelationMarker<R>>,
                      world: &World,
                      mut commands: Commands| {
                    for e in query_state.iter(world) {
                        commands.entity(e).remove_by_id(related_to);
                    }
                },
            );
            self.insert(RelationMarker::<R>(related_to, PhantomData));
            (self, related_to)
        }

        /// If the target has been set up properly (i.e. `make_relatable` was called on it), it has
        /// a marker component for each "relation" it can be used for. For example, entities that
        /// are edible (targets of `Eats` relation), have a `RelationMarker<Eats>(x)`. For example,
        /// if `target` is "apples", `x` is a component id that represents "eats apples". This
        /// function just copies that component id and inserts it onto `self`.
        fn relate<R: Component>(&mut self, target: Entity, data: R) -> &mut Self {
            let relation = **unsafe { self.world_mut() }
                .entity(target)
                .get::<RelationMarker<R>>()
                .expect("This entity hasn't been registered as relatable!");
            self.insert_dynamic(relation, data);
            self
        }
    }
}
