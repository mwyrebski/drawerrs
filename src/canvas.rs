use std::fmt;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.data.iter().fold(String::new(), |acc, row| {
            let line = row.into_iter().collect::<String>();
            format!("{}{}\n", acc, line)
        });
        write!(f, "{}", result)
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let data = vec![vec![' '; width]; height];
        Canvas {
            width,
            height,
            data,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }
    pub fn set(&mut self, x: usize, y: usize, ch: char) {
        self.data[y][x] = ch;
    }
    pub fn setp(&mut self, p: Point, ch: char) {
        self.data[p.1][p.0] = ch;
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn as_f64(self) -> (f64, f64) {
        (self.0 as f64, self.1 as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_to_string() {
        let c = Canvas::new(4, 2);
        assert_eq!("    \n    \n", c.to_string());
    }

    #[test]
    fn creates_canvas_with_new() {
        Canvas::new(2, 2);
    }
    #[test]
    fn new_matches_size() {
        let c = Canvas::new(5, 3);
        assert_eq!(5, c.width());
        assert_eq!(3, c.height());
    }
    #[test]
    fn set_matches_the_position() {
        let mut c = Canvas::new(5, 5);
        c.set(4, 2, '*');
        assert_eq!('*', c.get(4, 2));
    }
    #[test]
    fn get_works_on_extremes() {
        let c = Canvas::new(10, 5);
        for &(x, y) in [(0, 0), (9, 0), (9, 4), (0, 4)].iter() {
            assert_eq!(' ', c.get(x, y));
        }
    }
}
