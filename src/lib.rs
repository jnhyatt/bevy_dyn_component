use std::{alloc::Layout, any::TypeId, ptr::NonNull};

use bevy_app::App;
use bevy_ecs::{
    component::{Component, ComponentDescriptor, ComponentId, ComponentStorage},
    system::Resource,
    world::{EntityWorldMut, World},
};
use bevy_ptr::OwningPtr;
use bevy_utils::HashMap;

pub mod prelude {
    pub use super::{
        dynamic_components_plugin, DynamicComponentsEntityExt, DynamicComponentsWorldExt,
    };
}

pub fn dynamic_components_plugin(app: &mut App) {
    app.init_resource::<DynamicComponentRegistry>();
}

#[derive(Resource, Default, Debug)]
/// Stores the [`TypeId`] for each dynamic component registered with this API. This is used to
/// safely manipulate data underlying dynamic components.
pub struct DynamicComponentRegistry {
    component_types: HashMap<ComponentId, TypeId>,
}

pub trait DynamicComponentsWorldExt {
    /// Register a new dynamic component. The new component will have a data layout, destructor and
    /// component storage type matching `T`.
    fn dynamic_component<T: Component>(&mut self) -> ComponentId;
}

impl DynamicComponentsWorldExt for World {
    fn dynamic_component<T: Component>(&mut self) -> ComponentId {
        assert!(self.get_resource::<DynamicComponentRegistry>().is_some(), "`DynamicComponentRegistry` was not in the world! Make sure to add `dynamic_components_plugin` before using this API.");

        unsafe fn drop<T>(this: OwningPtr) {
            // SAFETY: this component was created as a `T`
            unsafe {
                this.drop_as::<T>();
            }
        }

        // SAFETY:
        // - `drop` drops values as `T`
        // - `T` is a `Component` which is `Send + Sync`
        let descriptor = unsafe {
            ComponentDescriptor::new_with_layout(
                std::any::type_name::<T>(),
                T::Storage::STORAGE_TYPE,
                Layout::new::<T>(),
                Some(drop::<T>),
            )
        };
        let id = self.init_component_with_descriptor(descriptor);

        // SAFETY (not unsafe): this is a brand new id, it can't be in the map yet
        self.resource_mut::<DynamicComponentRegistry>()
            .component_types
            .insert_unique_unchecked(id, TypeId::of::<T>());
        id
    }
}

pub trait DynamicComponentsEntityExt {
    /// Insert a dynamic component instance onto this entity. `T` must match the type used to
    /// register `component_id`.
    fn insert_dynamic<T: Component>(&mut self, component_id: ComponentId, data: T) -> &mut Self;
}

impl DynamicComponentsEntityExt for EntityWorldMut<'_> {
    fn insert_dynamic<T: Component>(
        &mut self,
        component_id: ComponentId,
        mut data: T,
    ) -> &mut Self {
        assert_eq!(
            *self
                .world()
                .get_resource::<DynamicComponentRegistry>()
                .expect("`DynamicComponentRegistry` was not in the world! Make sure to add `dynamic_components_plugin` before using this API.")
                .component_types
                .get(&component_id)
                .expect("Can't insert component with invalid id! The id must have been returned from `world.dynamic_component()` for this `World`."),
            TypeId::of::<T>(),
            "Incorrect type arguments supplied to `insert_dynamic`! `T` must be the same type `component_id` was created with."
        );
        let data_ptr = (&mut data as *mut T).cast::<u8>();

        // SAFETY: `data_ptr` is the address of `data` so can't be null
        let data_ptr = unsafe { NonNull::new_unchecked(data_ptr) };

        // SAFETY:
        // - `data_ptr` points to a valid `T`
        // - `data_ptr` is properly aligned since it came from a `&mut T`
        // - we have exclusive access to `data` and never borrow it again
        // - `data` lives until `insert_by_id` is done with it
        let data_ptr = unsafe { OwningPtr::new(data_ptr) };

        // Forget `data` so the ECS can take ownership of it
        std::mem::forget(data);

        // SAFETY:
        // - above assertion guarantees `component_id` is from this [`World`]
        // - `data_ptr` will be valid for the rest of the function
        unsafe { self.insert_by_id(component_id, data_ptr) }
    }
}
