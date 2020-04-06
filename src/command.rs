use crate::canvas::Point;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Command {
    Line { from: Point, to: Point },
    Rectangle { p1: Point, p2: Point },
    Circle { p: Point, r: usize },
    Canvas { width: usize, height: usize },
    Char(char),
    Read(String),
    Save(String),
    Info,
    Show,
    Quit,
}

fn try_parse_usize(s: &str) -> Result<usize, String> {
    s.parse::<usize>().map_err(|e| e.to_string())
}

impl Command {
    pub fn from(input: String) -> Result<Command, String> {
        let split: Vec<String> = input
            .trim()
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| {
                if i == 0 {
                    s.to_uppercase()
                } else {
                    s.to_string()
                }
            })
            .collect();
        let split: Vec<&str> = split.iter().map(String::as_ref).collect();
        if split.is_empty() {
            return Err("empty input".to_string());
        }
        let cmd = match split.as_slice() {
            ["LINE", x1, y1, x2, y2] => Command::Line {
                from: Point(try_parse_usize(x1)?, try_parse_usize(y1)?),
                to: Point(try_parse_usize(x2)?, try_parse_usize(y2)?),
            },
            ["RECT", x1, y1, x2, y2] | ["RECTANGLE", x1, y1, x2, y2] => Command::Rectangle {
                p1: Point(try_parse_usize(x1)?, try_parse_usize(y1)?),
                p2: Point(try_parse_usize(x2)?, try_parse_usize(y2)?),
            },
            ["CIRC", x, y, r] | ["CIRCLE", x, y, r] => Command::Circle {
                p: Point(try_parse_usize(x)?, try_parse_usize(y)?),
                r: try_parse_usize(r)?,
            },
            ["CANV", width, height] | ["CANVAS", width, height] => Command::Canvas {
                width: try_parse_usize(width)?,
                height: try_parse_usize(height)?,
            },
            ["CHAR", ch] => Command::Char(ch.parse::<char>().map_err(|e| e.to_string())?),
            ["READ", filename] => Command::Read(filename.to_string()),
            ["SAVE", filename] => Command::Save(filename.to_string()),
            ["INFO"] => Command::Info,
            ["SHOW"] => Command::Show,
            ["QUIT"] => Command::Quit,
            _ => return Err("unknown command".to_string()),
        };
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_variations(input: &str) -> Vec<String> {
        let capitalize = |s: &str| {
            let mut c = s.chars();
            c.next().unwrap().to_uppercase().chain(c).collect()
        };

        let mut vec: Vec<String> = Vec::new();
        for s in &[
            input.to_ascii_lowercase(),
            input.to_ascii_uppercase(),
            capitalize(input),
        ] {
            vec.push(s.clone());
            vec.push(format!(" {} ", s.clone()));
            vec.push(format!("\t{}\t\n", s.clone()));
            vec.push(s.split_whitespace().collect::<Vec<&str>>().join("\t"));
        }
        vec
    }

    #[test]
    fn line_is_parsed() {
        let expected = Command::Line {
            from: Point(1, 2),
            to: Point(3, 4),
        };
        for input in command_variations("line 1 2 3 4") {
            let cmd = Command::from(input).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn rect_is_parsed() {
        let expected = Command::Rectangle {
            p1: Point(1, 2),
            p2: Point(3, 4),
        };

        for input in [
            &command_variations("rect 1 2 3 4")[..],
            &command_variations("rectangle 1 2 3 4")[..],
        ]
        .concat()
        {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn circle_is_parsed() {
        let expected = Command::Circle {
            p: Point(1, 2),
            r: 3,
        };
        for input in [
            &command_variations("circ 1 2 3")[..],
            &command_variations("circle 1 2 3")[..],
        ]
        .concat()
        {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn canvas_is_parsed() {
        let expected = Command::Canvas {
            width: 100,
            height: 200,
        };
        for input in [
            &command_variations("canv 100 200")[..],
            &command_variations("canvas 100 200")[..],
        ]
        .concat()
        {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn char_is_parsed() {
        let expected = Command::Char('*');
        for input in command_variations("char *") {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn read_is_parsed() {
        for input in &[
            ("file_name", "read\tfile_name "),
            ("FileName", " READ FileName"),
            ("File_NAME", "Read \tFile_NAME \n"),
        ] {
            let expected = Command::Read(input.0.to_string());
            let cmd = Command::from(input.1.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn save_is_parsed() {
        for input in &[
            ("file_name", "save\tfile_name "),
            ("FileName", " SAVE FileName"),
            ("File_NAME", "Save \tFile_NAME \n"),
        ] {
            let expected = Command::Save(input.0.to_string());
            let cmd = Command::from(input.1.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn info_is_parsed() {
        for input in command_variations("info") {
            let expected = Command::Info;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn show_is_parsed() {
        for input in command_variations("show") {
            let expected = Command::Show;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn quit_is_parsed() {
        for input in command_variations("quit") {
            let expected = Command::Quit;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
}
