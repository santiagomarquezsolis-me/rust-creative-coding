use crossterm::{
    cursor,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, Result,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 24;

fn main() -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(EnableMouseCapture)?;

    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    let mut rng = rand::thread_rng();
    let mut columns: Vec<Option<usize>> = vec![None; WIDTH];

    loop {
        if poll(Duration::from_millis(50))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Esc {
                    break;
                }
            }
        }

        stdout.execute(SetBackgroundColor(Color::Black))?;
        stdout.execute(SetForegroundColor(Color::Green))?;

        for x in 0..WIDTH {
            let column = &mut columns[x];

            if let Some(y) = column {
                if *y > 0 {
                    execute!(
                        stdout,
                        cursor::MoveTo(x as u16, *y as u16),
                        Print(" ")
                    )?;
                }

                *y = if *y < HEIGHT { *y + 1 } else { 0 };

                execute!(
                    stdout,
                    cursor::MoveTo(x as u16, *y as u16),
                    Print(rng.gen_range(33..127) as u8 as char)
                )?;
            } else if rng.gen_bool(0.05) {
                *column = Some(0);
            }
        }

        stdout.flush()?;
        sleep(Duration::from_millis(100));
    }

    disable_raw_mode()?;
    stdout.execute(DisableMouseCapture)?;
    execute!(
        stdout,
        cursor::Show,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    )?;

    Ok(())
}
