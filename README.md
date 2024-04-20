# `bevy_dyn_component`

## Usage

Add `dynamic_components_plugin` to your `App`:
```rs
use bevy_dyn_component::prelude::*;
...
app.add_plugins(dynamic_components_plugin);
```
This initializes the component type registry needed for some of the unsafe sections. After that, you can use the `dynamic_component` and `insert_dynamic` APIs:
```rs
#[derive(Component)]
struct DynamicMarker;

let marker0 = world.dynamic_component::<DynamicMarker>();
world.entity_mut(e).insert_dynamic(marker0, DynamicMarker);
```

The component ID can also be used in the query builder API.

## Future of this crate

Now that Bevy supports dynamic components, it's likely that this crate will soon be obsolete. In the longer term, many of the things you can do with this crate is easier with [relations](https://github.com/bevyengine/bevy/issues/3742).

## Examples

### [`hello_world`](examples/hello_world.rs)
Demonstrates basic usage of the library, how to create dynamic components with a layout matching an existing component, how to insert them into entities, and how to use them in the query builder API.

### [`relations_at_home`](examples/relations_at_home.rs)
Demonstrates a simple way of using dynamic components as janky fragmenting relations. In my opinion, it doesn't work well as a general-purpose relations pattern, but for specific cases, it can be handy.

## Bevy Version

| bevy | bevy_dyn_component |
| ---- | ------------------ |
| 0.13 | 0.1                |

## Contribution

PRs welcome. If Bevy doesn't get good first-class support for this soon, I'll probably just keep updating this. It has missing APIs, for example, `take_dynamic`. I probably won't be adding this until I need it in a project, but I also won't hesitate to merge it in if someone else wants to tackle it.
