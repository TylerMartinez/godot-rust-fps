use gdnative::prelude::*;

// Import our "scripts" from the modules
mod controllers;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<controllers::player::Player>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);