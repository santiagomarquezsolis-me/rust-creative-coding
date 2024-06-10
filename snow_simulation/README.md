
# Snow Effect in Rust

This project implements a snow effect using the `ggez` game framework in Rust. The code simulates snowflakes falling from the top of the screen, accumulating at the bottom, and being affected by wind, a static moon is in the upper right corner too ;-)

## Dependencies

This project uses the following dependencies:

- `ggez`: A lightweight game framework for making 2D games with minimum friction.
- `rand`: A library for random number generation.

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
ggez = "0.6"
rand = "0.8"
```

## Code Explanation

### Constants

```rust
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SNOWFLAKE_COUNT: usize = 300;
const ACCUMULATION_SIZE: f32 = 2.0;
const WIND_CHANGE_INTERVAL: Duration = Duration::new(2, 0);
```

- `SCREEN_WIDTH` and `SCREEN_HEIGHT` define the dimensions of the window.
- `SNOWFLAKE_COUNT` specifies the number of snowflakes.
- `ACCUMULATION_SIZE` determines the size of accumulated snow.
- `WIND_CHANGE_INTERVAL` sets the interval for changing wind direction.

### Snowflake Struct

```rust
struct Snowflake {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    rotation: f32,
    rotation_speed: f32,
    color: Color,
    layer: usize,
}
```

This struct defines the properties of a snowflake, including its position, speed, size, rotation, color, and layer.

### MainState Struct

```rust
struct MainState {
    snowflakes: Vec<Snowflake>,
    wind: f32,
    accumulated_snow: Vec<Vec<bool>>,
    last_wind_change: Instant,
}
```

This struct holds the game state, including a vector of snowflakes, wind strength, accumulated snow grid, and the last time the wind direction was changed.

### MainState Implementation

#### Initialization

```rust
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let snowflakes = (0..SNOWFLAKE_COUNT)
            .map(|_| Snowflake {
                x: rng.gen_range(0.0..SCREEN_WIDTH),
                y: rng.gen_range(0.0..SCREEN_HEIGHT),
                speed: rng.gen_range(1.0..3.0),
                size: rng.gen_range(1.0..3.0),
                rotation: rng.gen_range(0.0..360.0),
                rotation_speed: rng.gen_range(-0.1..0.1),
                color: Color::WHITE,
                layer: rng.gen_range(0..3),
            })
            .collect();

        let accumulated_snow = vec![vec![false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        Ok(MainState {
            snowflakes,
            wind: 0.0,
            accumulated_snow,
            last_wind_change: Instant::now(),
        })
    }
```

- The `new` function initializes the game state by creating snowflakes with random properties and setting up the accumulated snow grid.

#### Update Function

```rust
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let now = Instant::now();
        if now - self.last_wind_change > WIND_CHANGE_INTERVAL {
            self.wind = rand::thread_rng().gen_range(-1.0..1.0);
            self.last_wind_change = now;
        }

        for snowflake in &mut self.snowflakes {
            snowflake.y += snowflake.speed;
            snowflake.x += self.wind + (snowflake.layer as f32 - 1.0) * 0.5;

            if snowflake.y > SCREEN_HEIGHT {
                snowflake.y = 0.0;
                snowflake.x = rand::thread_rng().gen_range(0.0..SCREEN_WIDTH);
            }

            if snowflake.x < 0.0 {
                snowflake.x = SCREEN_WIDTH;
            } else if snowflake.x > SCREEN_WIDTH {
                snowflake.x = 0.0;
            }
        }

        Ok(())
    }
```

- The `update` function updates the position of each snowflake based on its speed and the current wind strength.
- It also resets the position of snowflakes that go off-screen.

#### Draw Function

```rust
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        for snowflake in &self.snowflakes {
            let mesh = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Point2 { x: snowflake.x, y: snowflake.y },
                snowflake.size,
                0.1,
                snowflake.color,
            )?;
            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
```

- The `draw` function clears the screen and then draws each snowflake as a circle at its current position.

### Main Function

```rust
fn main() -> GameResult<()> {
    let (mut ctx, mut event_loop) = ContextBuilder::new("snow_effect", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Snow Effect"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let mut state = MainState::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
```

- The `main` function sets up the game window and starts the game loop.

## Running the Code

1. Ensure you have Rust and Cargo installed.
2. Add the `ggez` and `rand` dependencies to your `Cargo.toml`.
3. Run the project using `cargo run`.

## License

This project is licensed under the MIT License.
