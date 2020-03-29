use crate::canvas::Point;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Command {
    Line { from: Point, to: Point },
    Rect { p1: Point, p2: Point },
    Circ { p: Point, r: usize },
    Canv { width: usize, height: usize },
    Char(char),
    Read(String),
    Save(String),
    Info,
    Show,
    Quit,
}

impl Command {
    pub fn from(input: String) -> Result<Command, String> {
        let arg_err = || String::default();
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
            return Err(arg_err());
        }
        let cmd = match split.as_slice() {
            ["LINE", x1, y1, x2, y2] => {
                let x1 = x1.parse().unwrap();
                let y1 = y1.parse().unwrap();
                let x2 = x2.parse().unwrap();
                let y2 = y2.parse().unwrap();
                Command::Line {
                    from: Point(x1, y1),
                    to: Point(x2, y2),
                }
            }
            ["RECT", x1, y1, x2, y2] => {
                let x1 = x1.parse().unwrap();
                let y1 = y1.parse().unwrap();
                let x2 = x2.parse().unwrap();
                let y2 = y2.parse().unwrap();
                Command::Rect {
                    p1: Point(x1, y1),
                    p2: Point(x2, y2),
                }
            }
            ["CIRC", x, y, r] => {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                let r = r.parse().unwrap();
                Command::Circ { p: Point(x, y), r }
            }
            ["CANV", width, height] => {
                let width = width.parse().unwrap();
                let height = height.parse().unwrap();
                Command::Canv { width, height }
            }
            ["CHAR", ch] => {
                let ch = ch.parse().unwrap();
                Command::Char(ch)
            }
            ["READ", filename] => Command::Read(filename.to_string()),
            ["SAVE", filename] => Command::Save(filename.to_string()),
            ["INFO"] => Command::Info,
            ["SHOW"] => Command::Show,
            ["QUIT"] => Command::Quit,
            _ => return Err(arg_err()),
        };
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_is_parsed() {
        let expected = Command::Line {
            from: Point(1, 2),
            to: Point(3, 4),
        };
        for input in &["line\t1 2 3 4", "LINE 1 2 3 4", "Line  1  2  3\t4\n"] {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn rect_is_parsed() {
        let expected = Command::Rect {
            p1: Point(1, 2),
            p2: Point(3, 4),
        };
        for input in &["rect\t1 2 3 4", "RECT 1 2 3 4", "Rect  1  2  3\t4\n"] {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn circ_is_parsed() {
        let expected = Command::Circ {
            p: Point(1, 2),
            r: 3,
        };
        for input in &["circ\t1 2 3 ", " CIRC 1 2 3 ", "Circ  1  2  \t3\n"] {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn canv_is_parsed() {
        let expected = Command::Canv {
            width: 100,
            height: 200,
        };
        for input in &["canv\t100 200 ", " CANV 100 200", "Canv  100  \t200 \n"] {
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn char_is_parsed() {
        let expected = Command::Char('*');
        for input in &["char\t* ", " CHAR *", "Char \t* \n"] {
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
        for input in &["info", " \nINFO ", "  Info\t\n"] {
            let expected = Command::Info;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn show_is_parsed() {
        for input in &["show", " \nSHOW ", "  Show\t\n"] {
            let expected = Command::Show;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
    #[test]
    fn quit_is_parsed() {
        for input in &["quit", " \tQUIT ", "  Quit\t\n"] {
            let expected = Command::Quit;
            let cmd = Command::from(input.to_string()).unwrap();
            assert_eq!(expected, cmd);
        }
    }
}