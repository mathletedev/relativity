use macroquad::{input, prelude::*};

const SIZE: Vec2 = Vec2 { x: 40.0, y: 40.0 };
const ACCELERATION: f32 = 1.0;
const FRICTION: f32 = 0.9;

#[macroquad::main("Test")]
async fn main() {
	let mut position = Vec2 {
		x: screen_width() / 2.0 - SIZE.x / 2.0,
		y: screen_height() / 2.0 - SIZE.y / 2.0,
	};
	let mut velocity = Vec2 { x: 0.0, y: 0.0 };

	loop {
		clear_background(BLACK);

		if input::is_key_down(KeyCode::W) {
			velocity.y -= ACCELERATION;
		}
		if input::is_key_down(KeyCode::A) {
			velocity.x -= ACCELERATION;
		}
		if input::is_key_down(KeyCode::S) {
			velocity.y += ACCELERATION;
		}
		if input::is_key_down(KeyCode::D) {
			velocity.x += ACCELERATION;
		}

		position += velocity;
		velocity *= FRICTION;

		draw_rectangle(position.x, position.y, SIZE.x, SIZE.y, VIOLET);

		next_frame().await
	}
}
