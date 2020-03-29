mod canvas;
mod command;

use crate::canvas::Canvas;
use crate::canvas::Point;
use crate::command::Command;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    println!("drawerus");
    println!("");
    println!("Commands:");
    let mut canvas = Canvas::new(20, 10);
    let mut setchar = '*';
    loop {
        print!("> ");
        io::stdout().flush()?;

        if let Ok(cmd) = Command::from(read_line()) {
            match cmd {
                Command::Line { from, to } => {
                    let Point(x1, y1) = from;
                    let Point(x2, y2) = to;
                    let (fx1, fy1) = from.as_f64();
                    let (fx2, fy2) = to.as_f64();
                    let m = (fy2 - fy1) / (fx2 - fx1);

                    let form = |x, y| y as f64 - fy1 == m * (x as f64 - fx1);

                    canvas.setp(from, setchar);
                    canvas.setp(to, setchar);
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            if form(x, y) {
                                canvas.set(x, y, setchar);
                            }
                        }
                    }
                }
                Command::Rect { p1, p2 } => {
                    let Point(x1, y1) = p1;
                    let Point(x2, y2) = p2;
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            if x == x1 || x == x2 || y == y1 || y == y2 {
                                canvas.set(x, y, setchar);
                            }
                        }
                    }
                }
                Command::Circ { p, r } => {
                    let Point(x, y) = p;
                    let fr = r as f64;
                    let circle = |x, y| {
                        let fx = x as f64 - fr / 2.0;
                        let fy = y as f64 - fr / 2.0;
                        let dist = ((fx - fr).powi(2) + (fy - fr).powi(2)).sqrt();
                        dist > fr - 0.5 && dist < fr + 0.5
                    };

                    for x in x - r - 1..=x + r + 1 {
                        for y in y - r - 1..=y + r + 1 {
                            if circle(x, y) {
                                canvas.set(x, y, setchar);
                            }
                        }
                    }
                }
                Command::Canv { width, height } => {
                    canvas = Canvas::new(width, height);
                    println!("New canvas size {}x{}", width, height);
                }
                Command::Char(ch) => {
                    setchar = ch;
                    println!("Will use new char '{}'", setchar);
                }
                Command::Read(filename) => {}
                Command::Save(filename) => {
                    std::fs::write(filename, canvas.to_string())?;
                }
                Command::Info => println!("{}", canvas.info()),
                Command::Show => {
                    print!("{}", canvas);
                }
                Command::Quit => {
                    println!("Quitting...");
                    break;
                }
            }
        } else {
            println!("Invalid command.");
        }
    }
    Ok(())
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}
