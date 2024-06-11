use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, DrawParam};
use ggez::mint::Point2;
use rand::Rng;
use std::time::Duration;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const FIRE_PARTICLE_COUNT: usize = 300;
const GRAVITY: f32 = -0.1;

struct FireParticle {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    color: Color,
}

struct MainState {
    fire_particles: Vec<FireParticle>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let fire_particles = (0..FIRE_PARTICLE_COUNT)
            .map(|_| FireParticle {
                x: rng.gen_range(0.0..SCREEN_WIDTH),
                y: rng.gen_range(SCREEN_HEIGHT - 50.0..SCREEN_HEIGHT),
                speed: rng.gen_range(2.0..4.0),
                size: rng.gen_range(3.0..6.0),
                color: Color::from_rgb(rng.gen_range(200..255), rng.gen_range(100..150), 0),
            })
            .collect();

        Ok(MainState { fire_particles })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for particle in &mut self.fire_particles {
            particle.y += particle.speed;
            particle.x += particle.speed * 0.2; // Hacer que las partículas se dispersen
            particle.speed += GRAVITY; // Aplicar gravedad inversa

            if particle.y < 0.0 {
                // Reiniciar la partícula cuando alcanza la parte superior
                let mut rng = rand::thread_rng();
                particle.x = rng.gen_range(0.0..SCREEN_WIDTH);
                particle.y = SCREEN_HEIGHT;
                particle.speed = rng.gen_range(2.0..4.0);
                particle.size = rng.gen_range(3.0..6.0);
                particle.color = Color::from_rgb(rng.gen_range(200..255), rng.gen_range(100..150), 0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(30, 0, 0));

        // Dibujar el gradiente del cielo
        for i in 0..SCREEN_HEIGHT as usize {
            let color = Color::from_rgb((30 + (i as f32 / SCREEN_HEIGHT * 70.0) as u8).min(100), 0, 0);
            let rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, i as f32, SCREEN_WIDTH, 1.0),
                color,
            )?;
            canvas.draw(&rect, DrawParam::default());
        }

        // Dibujar la base de lava ardiente
        for i in (SCREEN_HEIGHT as usize - 50)..SCREEN_HEIGHT as usize {
            let color = Color::from_rgb(
                (200 + ((SCREEN_HEIGHT - i as f32) / 50.0 * 55.0) as u8).min(255),
                (50 + ((SCREEN_HEIGHT - i as f32) / 50.0 * 50.0) as u8).min(150),
                0,
            );
            let rect = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, i as f32, SCREEN_WIDTH, 1.0),
                color,
            )?;
            canvas.draw(&rect, DrawParam::default());
        }

        // Dibujar las partículas de fuego
        for particle in &self.fire_particles {
            let circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [particle.x, particle.y],
                particle.size,
                0.1,
                particle.color,
            )?;
            canvas.draw(&circle, DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("fire_effect", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Fire Effect"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
