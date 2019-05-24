extern crate crossterm;

use std::{thread, time};
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use crossterm::*;

use logic::field::Direction;
use logic::field::Field;

mod logic;

struct Bounds {
    col: u16,
    row: u16
}


struct Term {
    cursor: TerminalCursor,
    input: TerminalInput,
    terminal: Terminal,

    // need to save all even if they are not used. Else crossterm fails to work.
    #[allow(dead_code)]
    screen: RawScreen,
    #[allow(dead_code)]
    crossterm: Crossterm,
}

impl Term {
    fn new() -> Term {
        let crossterm = Crossterm::new();

        Term {
            cursor: crossterm.cursor(),
            input: crossterm.input(),
            terminal: crossterm.terminal(),
            screen: RawScreen::into_raw_mode().unwrap(),
            crossterm,
        }
    }
}

fn main() {
    let mut term: Term = Term::new();

    let bounds = init(&mut term);

    // setup field
    let mut score: u32 = 0;
    let mut field = Field::new(bounds.col as usize, bounds.row  as usize, ' ', 'X', 'O', 'G', 5).expect("Illegal sizes");
    draw_field(&field, &term.cursor);


    // input-loop
    let input = term.input; // prevent move of term into closure
    let ch = Arc::new(Mutex::new(KeyEvent::Delete));
    {
        let ch = Arc::clone(&ch);

        thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(50));

                let mut stdin = input.read_sync();

                if let Some(key_event) = stdin.next() {
                    match key_event {
                        InputEvent::Keyboard(event) => {
                            let new_key_event = event;

                            let mut ch_ptr = ch.lock().unwrap();
                            *ch_ptr = new_key_event;
                        },
                        _ => {}
                    }
                }


            }
        });
    }

    let mut end = false;

    let mut last: Option<Direction> = None;

    while !end
    {
        let speed: u64 = 200 - score as u64 * 2;
        let speed: u64 = if speed < 150 {150} else {speed};
        thread::sleep(time::Duration::from_millis(speed));

        let ch = ch.try_lock();

        let mut dir = match ch {
            Ok(m) => {
                match *m {
                    KeyEvent::Left => {
                        Some(Direction::LEFT)
                    },
                    KeyEvent::Right => {
                        Some(Direction::RIGHT)
                    },
                    KeyEvent::Up => {
                        Some(Direction::UP)
                    },
                    KeyEvent::Down => {
                        Some(Direction::DOWN)
                    },
                    _ => None
                }

            },
            _ => None
        };

        if dir.is_some() {
            last = dir.clone();
        }

        if dir.is_none() {
            dir = last.clone();
        }

        let (to_update, scored) = field.mov(dir);

        if scored {
            score += 1;
        }

        match to_update {
            Some(t) => update(t, score, &term.cursor),
            None => { if dir.is_some() { end = true; } }
        }
    }

    term.cursor.show().unwrap();
    RawScreen::disable_raw_mode().unwrap();
}

fn draw_field(field: &Field<char>, cursor: &TerminalCursor) {
    for (y, col) in field.get_field().iter().enumerate() {
        for (x, item) in col.iter().enumerate() {
            let item = *item;
            let res = cursor.goto(x as u16, y as u16);
            if res.is_ok() {
                print!("{}", item);
            }
        }
    }
}

fn update(points: LinkedList<(logic::field::Point, char)>, score: u32, cursor: &TerminalCursor) {
    for (point, chr) in points {
        let res = cursor.goto(point.0 as u16, point.1 as u16);
        if res.is_ok() {
            print!("{}", chr);
        }
    }
    let score: &str = &score.to_string()[0..];
    let res = cursor.goto(0, 0);
    if res.is_ok() {
        print!("{}", score);
    }
}

fn init(term: &mut Term) -> Bounds {

    // setup crossterm
    let res = term.cursor.hide();
    res.unwrap(); // panic as this application does not make sense if the terminal doesn't work

    let res = term.terminal.clear(ClearType::All);
    res.unwrap(); // panic as this application does not make sense if the terminal doesn't work


    let (width, height) = term.terminal.terminal_size();
    let bounds = Bounds {
        col: width,
        row: height,
    };

    return bounds
}