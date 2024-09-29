pub struct Quad {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Quad {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Quad {
        Quad {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct CLIRenderer {
    buffer: Vec<Vec<char>>,
}

impl CLIRenderer {
    pub fn new(width: usize) -> CLIRenderer {
        CLIRenderer {
            buffer: vec![vec![' '; width]; 1],
        }
    }

    pub fn to_string(&self) -> String {
        self.buffer
            .clone()
            .into_iter()
            .map(|inner| inner.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn place(&mut self, x: usize, y: usize, character: char) {
        if y >= self.buffer.len() {
            self.buffer.resize(
                y + 1,
                vec![' '; self.buffer.get(0).map_or(0, |row| row.len())],
            );
        }

        if x >= self.buffer[y].len() {
            self.buffer[y].resize(x + 1, ' ');
        }
        self.buffer[y][x] = character;
    }

    pub fn get(&mut self, x: usize, y: usize) -> char {
        if y >= self.buffer.len() {
            self.buffer.resize(
                y + 1,
                vec![' '; self.buffer.get(0).map_or(0, |row| row.len())],
            );
        }

        if x >= self.buffer[y].len() {
            self.buffer[y].resize(x + 1, ' ');
        }
        self.buffer[y][x]
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &'static str) {
        for i in 0..text.len() {
            self.place(x + i, y, text.as_bytes()[i].into());
        }
    }

    pub fn draw_vline(&mut self, x: usize, y: usize, height: usize, thick: bool) {
        for i in y..y + height {
            if self.get(x, i) == '─' {
                self.place(x, i, if thick { '╫' } else { '┼' });
            } else if self.get(x, i) == '═' {
                self.place(x, i, if thick { '║' } else { '╫' });
            } else {
                self.place(x, i, if thick { '║' } else { '│' })
            }
        }
    }

    pub fn draw_hline(&mut self, x: usize, y: usize, width: usize, thick: bool) {
        for i in x..x + width {
            if self.get(i, y) == '│' {
                self.place(i, y, if thick { '╪' } else { '┼' });
            } else if self.get(i, y) == '║' {
                self.place(i, y, if thick { '║' } else { '╫' });
            } else {
                self.place(i, y, if thick { '═' } else { '─' })
            }
        }
    }

    pub fn draw_quad(&mut self, quad: Quad, thick: bool) {
        for x in quad.x..quad.x + quad.width {
            for y in quad.y..quad.y + quad.height {
                if x == quad.x && y == quad.y {
                    self.place(x, y, if thick { '╔' } else { '┌' });
                } else if x == quad.x + quad.width - 1 && y == quad.y {
                    self.place(x, y, if thick { '╗' } else { '┐' });
                } else if x == quad.x && y == quad.y + quad.height - 1 {
                    self.place(x, y, if thick { '╚' } else { '└' });
                } else if x == quad.x + quad.width - 1 && y == quad.y + quad.height - 1 {
                    self.place(x, y, if thick { '╝' } else { '┘' });
                } else if y == quad.y || y == quad.y + quad.height - 1 {
                    self.draw_hline(x, y, 1, thick);
                } else if x == quad.x || x == quad.x + quad.width - 1 {
                    self.draw_vline(x, y, 1, thick);
                }
            }
        }
    }
}
