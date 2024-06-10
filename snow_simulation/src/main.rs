use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, DrawParam};
use ggez::mint::Point2;
use rand::Rng;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SNOWFLAKE_COUNT: usize = 300;
const ACCUMULATION_SIZE: f32 = 2.0;
const WIND_CHANGE_INTERVAL: Duration = Duration::new(2, 0);

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

struct MainState {
    snowflakes: Vec<Snowflake>,
    wind: f32,
    accumulated_snow: Vec<Vec<bool>>,
    last_wind_change: Instant,
}

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
                rotation_speed: rng.gen_range(-1.0..1.0),
                color: Color::from_rgb(200, 200, rng.gen_range(200..255)),
                layer: rng.gen_range(0..3), // Tres capas de copos de nieve
            })
            .collect();

        let accumulated_snow = vec![vec![false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
        let wind = 0.5;
        let last_wind_change = Instant::now();

        Ok(MainState { snowflakes, wind, accumulated_snow, last_wind_change })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.last_wind_change.elapsed() >= WIND_CHANGE_INTERVAL {
            let mut rng = rand::thread_rng();
            self.wind = rng.gen_range(-1.0..1.0);
            self.last_wind_change = Instant::now();
        }

        for snowflake in &mut self.snowflakes {
            snowflake.y += snowflake.speed;
            snowflake.x += self.wind * (snowflake.layer as f32 + 1.0); // Capas de copos con diferentes influencias del viento
            snowflake.rotation += snowflake.rotation_speed;

            if snowflake.x >= SCREEN_WIDTH {
                snowflake.x = 0.0;
            } else if snowflake.x < 0.0 {
                snowflake.x = SCREEN_WIDTH - 1.0;
            }

            if snowflake.y >= SCREEN_HEIGHT - 1.0 || self.accumulated_snow[snowflake.y as usize][snowflake.x as usize] {
                if snowflake.y >= SCREEN_HEIGHT - 1.0 {
                    snowflake.y = SCREEN_HEIGHT - 1.0;
                }
                self.accumulated_snow[snowflake.y as usize][snowflake.x as usize] = true;
                let mut rng = rand::thread_rng();
                snowflake.x = rng.gen_range(0.0..SCREEN_WIDTH);
                snowflake.y = 0.0;
                snowflake.speed = rng.gen_range(1.0..3.0);
                snowflake.size = rng.gen_range(1.0..3.0);
                snowflake.rotation = rng.gen_range(0.0..360.0);
                snowflake.rotation_speed = rng.gen_range(-1.0..1.0);
                snowflake.color = Color::from_rgb(200, 200, rng.gen_range(200..255));
                snowflake.layer = rng.gen_range(0..3);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(0, 0, 30));

        // Dibujar el gradiente del cielo
        for i in 0..SCREEN_HEIGHT as usize {
            let color = Color::from_rgb(0, 0, (30 + (i as f32 / SCREEN_HEIGHT * 30.0) as u8).min(60));
            let rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, i as f32, SCREEN_WIDTH, 1.0),
                color,
            )?;
            canvas.draw(&rect, DrawParam::default());
        }

        // Dibujar la luna
        let moon = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [700.0, 100.0],
            50.0,
            0.1,
            Color::from_rgb(230, 230, 250),
        )?;
        canvas.draw(&moon, DrawParam::default());

        for snowflake in &self.snowflakes {
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [0.0, 0.0],
                snowflake.size,
                0.1,
                snowflake.color,
            )?;
            let draw_param = DrawParam::default()
                .dest(Point2 { x: snowflake.x, y: snowflake.y })
                .rotation(snowflake.rotation.to_radians())
                .offset(Point2 { x: 0.5, y: 0.5 });
            canvas.draw(&circle, draw_param);
        }

        let snowflake_color = Color::from_rgb(255, 255, 255);
        for y in 0..SCREEN_HEIGHT as usize {
            for x in 0..SCREEN_WIDTH as usize {
                if self.accumulated_snow[y][x] {
                    let pixel = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        Rect::new(x as f32, y as f32, ACCUMULATION_SIZE, ACCUMULATION_SIZE),
                        snowflake_color,
                    )?;
                    canvas.draw(&pixel, DrawParam::default());
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("snow_effect", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Snow Effect"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
