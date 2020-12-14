use gdnative::prelude::*;
use gdnative::api::{InputEventMouseMotion};

/// The Player controller "class"
#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    #[property(default = 0.03)]
    mouse_sensitivity: f32,
    head: Option<Ref<Spatial, Shared>>
}

#[methods]
impl Player {
    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            mouse_sensitivity: 0.03,
            head: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody) {
        godot_print!("Hello from Player Controller");

        // Capture mouse character
        let input = Input::godot_singleton();
        input.set_mouse_mode(2);
        
        // Save a reference to the head node
        let head = unsafe { owner.get_node("./head").unwrap().assume_safe().cast::<Spatial>().unwrap().claim() };
        self.head = Some(head);
    }

    #[export]
    fn _input(&mut self, owner: &KinematicBody, event: Ref<InputEvent>) {
        godot_print!("Input Fired!");

        if let Some(e) = event.cast::<InputEventMouseMotion>() {
            let mouse_event = unsafe { e.assume_safe() };
            owner.rotate_y((-mouse_event.relative().x * self.mouse_sensitivity).to_radians().into());
            
            let head = unsafe { self.head.unwrap().assume_safe() };
            head.rotate_x((-mouse_event.relative().y * self.mouse_sensitivity).to_radians().into());
        }
    }
}