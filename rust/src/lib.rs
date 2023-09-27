use godot::{prelude::*, engine::{CharacterBody2D, CharacterBody2DVirtual}};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Ball {
    #[export]
    pub direction: Vector2,
    // speed: f32,
    #[base]
    body: Base<CharacterBody2D>
}

#[godot_api]
impl CharacterBody2DVirtual for Ball {
    fn init(body: Base<CharacterBody2D>) -> Self {
        Self { 
            direction: Vector2::new(500.0, 500.0), 
            // speed: 200.0, 
            body 
        }
    }
    fn physics_process(&mut self, delta: f64) {
        if let Some(collision) = self.body.move_and_collide(self.direction * delta as f32) {
            let reflect = collision.get_remainder().reflect(collision.get_normal());
            self.direction = self.direction.reflect(collision.get_normal());
            godot_print!("---[gdext]--- \nReflect: {:?} \nDirection: {:?} \n-------------", reflect, self.direction);
            self.body.move_and_collide(reflect);
        } else {
            return
        }
    }
}

#[godot_api]
impl Ball {}