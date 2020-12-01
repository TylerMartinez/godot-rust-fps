use gdnative::prelude::*;

/// The Player controller "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Player;

#[methods]
impl Player {
    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        Player
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello from Player Controller");
    }
}