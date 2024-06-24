
# Raindrop Simulation in Rust

This project is a simple graphical simulation of raindrops falling across the screen, implemented in Rust using the `ggez` game engine library.

## Features

- **Random Raindrop Generation**: Raindrops are randomly generated with different positions, sizes, and speeds.
- **Gravity Effect**: Raindrops accelerate downwards due to gravity.
- **Screen Wrapping**: Raindrops wrap around the screen edges, reappearing on the opposite side to create a continuous effect.
- **Diagonal Fall**: Raindrops fall diagonally for a more dynamic visual effect.

## Requirements

- Rust programming language
- `ggez` game engine library
- `rand` library

## Installation

1. Ensure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
2. Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
ggez = "0.9.3"
rand = "0.8"
```

3. Clone this repository:

```sh
git clone https://github.com/yourusername/raindrop_simulation.git
cd raindrop_simulation
```

4. Build and run the project:

```sh
cargo run
```

## Code Overview

### main.rs

The main Rust file that contains the implementation of the raindrop simulation.

#### Imports

```rust
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, DrawParam};
use ggez::mint::Point2;
use rand::Rng;
use std::time::Duration;
```

These imports bring in the necessary modules from `ggez`, `rand`, and the standard library.

#### Constants

```rust
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const RAIN_DROP_COUNT: usize = 300;
const GRAVITY: f32 = 0.1;
```

These constants define the screen dimensions, the number of raindrops, and the gravity value.

#### Struct Definitions

```rust
struct Raindrop {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    color: Color,
}

struct MainState {
    raindrops: Vec<Raindrop>,
}
```

- `Raindrop`: Represents a single raindrop with properties for position, speed, size, and color.
- `MainState`: Holds the state of the game, including a vector of `Raindrop` instances.

#### Implementations

- `MainState::new`: Initializes the game state with randomly generated raindrops.

```rust
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let raindrops = (0..RAIN_DROP_COUNT)
            .map(|_| Raindrop {
                x: rng.gen_range(0.0..SCREEN_WIDTH),
                y: rng.gen_range(0.0..SCREEN_HEIGHT),
                speed: rng.gen_range(4.0..8.0),
                size: rng.gen_range(1.0..3.0),
                color: Color::from_rgb(0, 0, rng.gen_range(150..255)),
            })
            .collect();

        Ok(MainState { raindrops })
    }
}
```

- `EventHandler for MainState`: Implements the `update` and `draw` methods.

```rust
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for raindrop in &mut self.raindrops {
            raindrop.y += raindrop.speed;
            raindrop.x += raindrop.speed * 0.5; // Make the drops fall diagonally
            raindrop.speed += GRAVITY; // Apply gravity

            if raindrop.x >= SCREEN_WIDTH {
                raindrop.x = 0.0;
            } else if raindrop.x < 0.0 {
                raindrop.x = SCREEN_WIDTH - 1.0;
            }

            if raindrop.y >= SCREEN_HEIGHT {
                raindrop.y = SCREEN_HEIGHT - 1.0;
                raindrop.speed = 0.0; // Eliminate bounce

                // Reset the drop immediately after "bounce"
                let mut rng = rand::thread_rng();
                raindrop.x = rng.gen_range(0.0..SCREEN_WIDTH);
                raindrop.y = 0.0;
                raindrop.speed = rng.gen_range(4.0..8.0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        for raindrop in &self.raindrops {
            let rectangle = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(raindrop.x, raindrop.y, raindrop.size, raindrop.size * 2.0),
                raindrop.color,
            )?;
            graphics::draw(ctx, &rectangle, DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}
```

The `update` method moves the raindrops and applies gravity, while the `draw` method renders them on the screen.

## Usage

Run the program to see the raindrop simulation in action. The raindrops will fall diagonally, accelerating due to gravity and wrapping around the screen edges.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
