extends CharacterBody2D

var direction := Vector2.ZERO;

func _ready() -> void:
	direction = Vector2(500, 500);


func _physics_process(delta: float) -> void:
	var collision :KinematicCollision2D = move_and_collide(direction * delta);
	
	if collision:
		var reflect : Vector2 = collision.get_remainder().bounce(collision.get_normal());
		direction = direction.bounce(collision.get_normal());
		print("---[GDScript]--- \n", "Reflect: ", reflect, "\n", "Direction: ", direction, "\n", "----------------")
		move_and_collide(reflect);
