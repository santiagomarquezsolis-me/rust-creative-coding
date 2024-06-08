use ggez::{conf, event, graphics, Context, GameResult};
use rand::Rng;
use std::convert::TryInto;

const FIRE_WIDTH: usize = 50;
const FIRE_HEIGHT: usize = 50;
const PIXEL_SIZE: f32 = 10.0;
const SCREEN_WIDTH: f32 = (FIRE_WIDTH as f32) * PIXEL_SIZE;
const SCREEN_HEIGHT: f32 = (FIRE_HEIGHT as f32) * PIXEL_SIZE;

struct FireSimulation {
    fire_pixels_array: Vec<u8>,
    fire_colors_palette: Vec<graphics::Color>,
}

impl FireSimulation {
    fn new() -> Self {
        let fire_colors_palette = vec![
            graphics::Color::from_rgb(7, 7, 7),
            graphics::Color::from_rgb(31, 7, 7),
            graphics::Color::from_rgb(47, 15, 7),
            graphics::Color::from_rgb(71, 15, 7),
            graphics::Color::from_rgb(87, 23, 7),
            graphics::Color::from_rgb(103, 31, 7),
            graphics::Color::from_rgb(119, 31, 7),
            graphics::Color::from_rgb(143, 39, 7),
            graphics::Color::from_rgb(159, 47, 7),
            graphics::Color::from_rgb(175, 63, 7),
            graphics::Color::from_rgb(191, 71, 7),
            graphics::Color::from_rgb(199, 71, 7),
            graphics::Color::from_rgb(223, 79, 7),
            graphics::Color::from_rgb(223, 87, 7),
            graphics::Color::from_rgb(223, 87, 7),
            graphics::Color::from_rgb(215, 95, 7),
            graphics::Color::from_rgb(215, 95, 7),
            graphics::Color::from_rgb(215, 103, 15),
            graphics::Color::from_rgb(207, 111, 15),
            graphics::Color::from_rgb(207, 119, 15),
            graphics::Color::from_rgb(207, 127, 15),
            graphics::Color::from_rgb(207, 135, 23),
            graphics::Color::from_rgb(199, 135, 23),
            graphics::Color::from_rgb(199, 143, 23),
            graphics::Color::from_rgb(199, 151, 31),
            graphics::Color::from_rgb(191, 159, 31),
            graphics::Color::from_rgb(191, 159, 31),
            graphics::Color::from_rgb(191, 167, 39),
            graphics::Color::from_rgb(191, 167, 39),
            graphics::Color::from_rgb(191, 175, 47),
            graphics::Color::from_rgb(183, 175, 47),
            graphics::Color::from_rgb(183, 183, 47),
            graphics::Color::from_rgb(183, 183, 55),
            graphics::Color::from_rgb(207, 207, 111),
            graphics::Color::from_rgb(223, 223, 159),
            graphics::Color::from_rgb(239, 239, 199),
            graphics::Color::from_rgb(255, 255, 255),
        ];

        let fire_pixels_array = vec![0; FIRE_WIDTH * FIRE_HEIGHT];
        let mut sim = FireSimulation {
            fire_pixels_array,
            fire_colors_palette,
        };
        sim.create_fire_source();
        sim
    }

    fn create_fire_source(&mut self) {
        for column in 0..FIRE_WIDTH {
            let pixel_index = (FIRE_WIDTH * FIRE_HEIGHT - FIRE_WIDTH) + column;
            self.fire_pixels_array[pixel_index] = 36;
        }
    }

    fn update_fire_intensity(&mut self, current_pixel_index: usize) {
        let below_pixel_index = current_pixel_index + FIRE_WIDTH;
        if below_pixel_index >= FIRE_WIDTH * FIRE_HEIGHT {
            return;
        }

        let decay = rand::thread_rng().gen_range(0..3) as usize; // Convertir decay a usize
        let below_pixel_fire_intensity = self.fire_pixels_array[below_pixel_index] as i32;
        let mut new_fire_intensity = below_pixel_fire_intensity - decay as i32;

        if new_fire_intensity < 0 {
            new_fire_intensity = 0;
        }

        if current_pixel_index >= decay {
            self.fire_pixels_array[current_pixel_index - decay] = new_fire_intensity as u8;
        }
    }

    fn calculate_fire_propagation(&mut self) {
        for row in 0..FIRE_HEIGHT {
            for column in 0..FIRE_WIDTH {
                let pixel_index = column + (FIRE_WIDTH * row);
                self.update_fire_intensity(pixel_index);
            }
        }
    }

    fn draw_fire(&self, ctx: &mut Context) -> GameResult {
        for row in 0..FIRE_HEIGHT {
            for column in 0..FIRE_WIDTH {
                let pixel_index = column + (FIRE_WIDTH * row);
                let fire_intensity = self.fire_pixels_array[pixel_index] as usize;
                let color = self.fire_colors_palette[fire_intensity];
                let rectangle = graphics::Rect::new(
                    column as f32 * PIXEL_SIZE,
                    row as f32 * PIXEL_SIZE,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                );
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rectangle, color)?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }
}

impl event::EventHandler for FireSimulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.calculate_fire_propagation();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        self.draw_fire(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Fire Simulation", "ggez")
        .window_setup(conf::WindowSetup::default().title("Fire Simulation"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?; // No usar referencias mutables aquí
    let sim = FireSimulation::new(); // No usar referencia mutable aquí
    event::run(ctx, event_loop, sim) // Pasar los parámetros sin referencia mutable
}
