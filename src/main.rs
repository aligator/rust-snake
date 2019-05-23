extern crate crossterm;

use std::{thread, time};
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

use crossterm::*;

use logic::field::Direction;
use logic::field::Field;

mod logic;

struct Bounds {
    col: i32,
    row: i32
}

fn main() {
    // setup crossterm
    let crossterm = Crossterm::new();
    let screen = RawScreen::into_raw_mode();
    let terminal = crossterm.terminal();
    let mut input = crossterm.input();

    // setup field
    let bounds = init();
    let mut score: i32 = 0;
    let mut field = Field::new(bounds.col as usize, bounds.row  as usize, ' ', 'X', 'O', 'G', 5).expect("Illegal sizes");
    draw_field(&field);

    // input-loop
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
                            let new_keyEvent = event;

                            let mut ch_ptr = ch.lock().unwrap();
                            *ch_ptr = new_keyEvent;
                        },
                        _ => {}
                    }
                }


            }
        });
    }

    let mut end = false;

    let mut last: Option<char> = None;

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
                        println!("left");
                        Some(Direction::LEFT)
                    },
                    KeyEvent::Right => {
                        println!("right");
                        Some(Direction::RIGHT)
                    },
                    KeyEvent::Up => {
                        println!("up");
                        Some(Direction::UP)
                    },
                    KeyEvent::Down => {
                        println!("down");
                        Some(Direction::DOWN)
                    },
                    _ => None
                }

            },
            _ => None
        };
        /*
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
                    Some(t) => update(t, score),
                    None => {if dir.is_some() {end = true;}}
                }
                */

    }

    //endwin();
}

fn draw_field(field: &Field<char>) {
    for (y, col) in field.get_field().iter().enumerate() {
        for (x, item) in col.iter().enumerate() {
            let item = *item;
            //mvaddch(y as i32, x as i32, item as chtype);
        }
    }
/*
    for ((x, y), chr) in field.get_snake_with_chars() {
        mvaddch(y as i32, x as i32, chr as chtype);
    }

  */
}

fn update(points: LinkedList<(logic::field::Point, char)>, score: i32) {
    for (point, chr) in points {
        //mvaddch(point.1 as i32, point.0 as i32, chr as chtype);

    }
    let score: &str = &score.to_string()[0..];
    //mvprintw(0, 0, score);
    //refresh();
}

fn init() -> Bounds {
    /*let locale_conf = LcCategory::all;
    setlocale(locale_conf, "de_DE.UTF-8");

    let window = initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    start_color();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    Bounds {
        col: getmaxx(window),
        row: getmaxy(window)
    }*/

    Bounds {
        col: 100,
        row: 100
    }
}
