use piston_window::*;
use simpletowerdefense::game::Game;
use simpletowerdefense::tower::Tower;

pub struct GameState {
    game: Game,
    tower: Tower,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            tower: Tower::new(10, 200, (400.0, 300.0)),
        }
    }

    fn handle_input(&mut self, button: &Button, _is_press: bool) -> bool {
        match *button {
            Button::Keyboard(Key::Escape) => {
                // Save game before exiting
                if let Err(e) = self.game.save_game() {
                    eprintln!("Failed to save game on exit: {}", e);
                }
                true // Signal to exit the game
            }
            Button::Keyboard(Key::D) => {
                self.game.try_upgrade_tower_damage(&mut self.tower);
                false
            }
            Button::Keyboard(Key::F) => {
                self.game.try_upgrade_tower_fire_rate(&mut self.tower);
                false
            }
            _ => false,
        }
    }

    fn draw(&mut self, c: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        // Draw tower range indicator
        let range = self.tower.get_range() as f64;
        let tower_pos = self.tower.get_position();
        let transform = c.transform.trans(tower_pos.0, tower_pos.1);
        
        // Draw range circle
        for angle in (0..360).step_by(5) {
            let rad = angle as f64 * std::f64::consts::PI / 180.0;
            let x = rad.cos() * range;
            let y = rad.sin() * range;
            rectangle(
                [0.5, 0.5, 0.5, 0.2],
                [-1.0, -1.0, 2.0, 2.0],
                transform.trans(x, y),
                g,
            );
        }

        // Draw the tower
        let tower_rect = rectangle::rectangle_by_corners(
            tower_pos.0 - 25.0,
            tower_pos.1 - 25.0,
            tower_pos.0 + 25.0,
            tower_pos.1 + 25.0,
        );
        rectangle([1.0, 0.5, 0.0, 1.0], tower_rect, c.transform, g);

        // Draw bullets
        for bullet in self.game.get_bullets() {
            let pos = bullet.get_position();
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [pos.0, pos.1, 5.0, 5.0],
                c.transform,
                g,
            );
        }

        // Draw enemies with health bars
        for enemy in self.game.get_enemies() {
            let pos = enemy.get_position();
            
            // Draw enemy
            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [pos.0 - 15.0, pos.1 - 15.0, 30.0, 30.0],
                c.transform,
                g,
            );

            // Draw health bar background
            rectangle(
                [0.5, 0.0, 0.0, 1.0],
                [pos.0 - 20.0, pos.1 - 25.0, 40.0, 5.0],
                c.transform,
                g,
            );

            // Draw health bar
            let health_percentage = enemy.get_health_percentage();
            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [pos.0 - 20.0, pos.1 - 25.0, 40.0 * health_percentage, 5.0],
                c.transform,
                g,
            );
        }

        // Draw UI elements
        let padding = 10.0;
        let bar_height = 20.0;
        let y_spacing = 30.0;

        // Score (blue)
        let score = self.game.get_score();
        rectangle(
            [0.0, 0.0, 1.0, 1.0],
            [padding, padding, score as f64 * 2.0, bar_height],
            c.transform,
            g,
        );

        // Satoshis (gold)
        let satoshis = self.game.get_currency();
        rectangle(
            [1.0, 0.84, 0.0, 1.0],
            [padding, padding + y_spacing, satoshis as f64 * 2.0, bar_height],
            c.transform,
            g,
        );

        // Damage upgrade cost (red) with 'D' indicator
        let damage_cost = self.tower.get_damage_upgrade_cost();
        rectangle(
            [1.0, 0.0, 0.0, 0.8],
            [padding, padding + y_spacing * 2.0, damage_cost as f64, bar_height],
            c.transform,
            g,
        );
        // 'D' indicator
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [padding + damage_cost as f64 + 5.0, padding + y_spacing * 2.0, 10.0, bar_height],
            c.transform,
            g,
        );

        // Fire rate upgrade cost (yellow) with 'F' indicator
        let fire_rate_cost = self.tower.get_fire_rate_upgrade_cost();
        rectangle(
            [1.0, 1.0, 0.0, 0.8],
            [padding, padding + y_spacing * 3.0, fire_rate_cost as f64, bar_height],
            c.transform,
            g,
        );
        // 'F' indicator
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [padding + fire_rate_cost as f64 + 5.0, padding + y_spacing * 3.0, 10.0, bar_height],
            c.transform,
            g,
        );

        // Draw current upgrade levels
        let level_indicator_size = 5.0;
        let level_spacing = 8.0;

        // Damage level indicators
        for i in 0..self.tower.get_damage_level() {
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [
                    padding + (i as f64 * level_spacing),
                    padding + y_spacing * 2.0 + bar_height + 5.0,
                    level_indicator_size,
                    level_indicator_size,
                ],
                c.transform,
                g,
            );
        }

        // Fire rate level indicators
        for i in 0..self.tower.get_fire_rate_level() {
            rectangle(
                [1.0, 1.0, 0.0, 1.0],
                [
                    padding + (i as f64 * level_spacing),
                    padding + y_spacing * 3.0 + bar_height + 5.0,
                    level_indicator_size,
                    level_indicator_size,
                ],
                c.transform,
                g,
            );
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tower Defense", [800, 600])
        .exit_on_esc(false)
        .build()
        .unwrap();

    let mut game_state = GameState::new();
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(_args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                game_state.draw(&c, g);
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if game_state.handle_input(&Button::Keyboard(key), true) {
                break;
            }
        }

        if let Some(_args) = e.update_args() {
            game_state.game.update(&mut game_state.tower);
        }
    }
}
