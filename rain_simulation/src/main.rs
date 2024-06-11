use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, DrawParam};
use ggez::mint::Point2;
use rand::Rng;
use std::time::Duration;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const RAIN_DROP_COUNT: usize = 300;
const GRAVITY: f32 = 0.1;

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

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for raindrop in &mut self.raindrops {
            raindrop.y += raindrop.speed;
            raindrop.x += raindrop.speed * 0.5; // Hacer que las gotas caigan en diagonal
            raindrop.speed += GRAVITY; // Aplicar gravedad

            if raindrop.x >= SCREEN_WIDTH {
                raindrop.x = 0.0;
            } else if raindrop.x < 0.0 {
                raindrop.x = SCREEN_WIDTH - 1.0;
            }

            if raindrop.y >= SCREEN_HEIGHT {
                raindrop.y = SCREEN_HEIGHT - 1.0;
                raindrop.speed = 0.0; // Eliminar el rebote

                // Reiniciar la gota inmediatamente después del "rebote"
                let mut rng = rand::thread_rng();
                raindrop.x = rng.gen_range(0.0..SCREEN_WIDTH);
                raindrop.y = 0.0;
                raindrop.speed = rng.gen_range(4.0..8.0);
                raindrop.size = rng.gen_range(1.0..3.0);
                raindrop.color = Color::from_rgb(0, 0, rng.gen_range(150..255));
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

        for raindrop in &self.raindrops {
            let line = Mesh::new_line(
                ctx,
                &[
                    Point2 { x: raindrop.x, y: raindrop.y },
                    Point2 { x: raindrop.x, y: raindrop.y + raindrop.size * 5.0 },
                ],
                1.0,
                raindrop.color,
            )?;
            canvas.draw(&line, DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("rain_effect", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Rain Effect"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
