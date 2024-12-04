extern crate piston_window;

use piston_window::*;

mod tower;
mod enemy;
mod game;

struct MainState {
    tower_position: (f32, f32),
}

impl MainState {
    pub fn new() -> Self {
        Self {
            tower_position: (400.0, 300.0), // Center of the screen
        }
    }
}

impl MainState {
    fn draw(&self, c: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g); // Clear the screen to black

        // Draw the tower
        let tower_rect = rectangle::rectangle_by_corners(
            (self.tower_position.0 - 25.0).into(),
            (self.tower_position.1 - 25.0).into(),
            (self.tower_position.0 + 25.0).into(),
            (self.tower_position.1 + 25.0).into(),
        );
        // Draw the tower in orange
        rectangle([1.0, 0.5, 0.0, 1.0], tower_rect, c.transform, g); // Draw an orange rectangle
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Simple Tower Defense", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let tower = tower::Tower::new(10, 100); // Example values for damage and range
    println!("Tower Damage: {}, Range: {}", tower.get_damage(), tower.get_range()); // Display tower details using getter methods

    let state = MainState::new();
    let mut game = game::Game::new(); // Create an instance of Game
    game.start_game(); // Start the game

    // Example of applying damage to an enemy
    let mut enemy = enemy::Enemy::new(100, 5);
    println!("Enemy Health: {}, Speed: {}", enemy.get_health(), enemy.get_speed()); // Display enemy details using getter methods
    enemy.take_damage(10); // Apply damage to the enemy
    game.increment_score(10); // Increment score by 10 for demonstration

    game.pause_game(); // Call pause_game
    game.resume_game(); // Call resume_game
    game.end_game(); // Call end_game

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            state.draw(&c, g);
        });
    }
}
