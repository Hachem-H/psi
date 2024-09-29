#[derive(Clone, Copy)]
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

    fn expand(&mut self, x: usize, y: usize) {
        if y >= self.buffer.len() {
            self.buffer.resize(
                y + 1,
                vec![' '; self.buffer.get(0).map_or(0, |row| row.len())],
            );
        }

        if x >= self.buffer[y].len() {
            self.buffer[y].resize(x + 1, ' ');
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
        self.expand(x, y);
        self.buffer[y][x] = character;
    }

    pub fn get(&mut self, x: usize, y: usize) -> char {
        self.expand(x, y);
        self.buffer[y][x]
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &'static str) {
        self.expand(x + text.len(), y);
        for i in 0..text.len() {
            self.place(x + i, y, text.as_bytes()[i].into());
        }
    }

    pub fn draw_vline(&mut self, x: usize, y: usize, height: usize, thick: bool) {
        self.expand(x, y + height);

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
        self.expand(x + width, y);

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
        self.expand(quad.x + quad.width, quad.y + quad.height);

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

    pub fn connect(&mut self, quad: Quad, point: (usize, usize)) {
        enum Directions {
            Bottom,
            BottomLeft,
            BottomRight,

            Top,
            TopLeft,
            TopRight,

            Right,
            Left,
            None,
        }

        let center_x = (((quad.x + quad.width) as f32 / 2.0).ceil()) as usize;
        let center_y = (((quad.y + quad.height) as f32 / 2.0).ceil()) as usize;

        let point_x = point.0;
        let point_y = point.1;

        let mut edge_midpoint_x = 0usize;
        let mut edge_midpoint_y = 0usize;

        let line_midpoint_x = center_x;
        let line_midpoint_y = point_y;

        let mut direction = Directions::None;

        if point_y > center_y && point_x == center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y + quad.height - 1;
            direction = Directions::Bottom;
        } else if point_y > center_y && point_x < center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y + quad.height - 1;
            direction = Directions::BottomLeft;
        } else if point_y > center_y && point_x > center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y + quad.height - 1;
            direction = Directions::BottomRight;
        } else if point_y < center_y && point_x == center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y;
            direction = Directions::Top;
        } else if point_y < center_y && point_x < center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y;
            direction = Directions::TopLeft;
        } else if point_y < center_y && point_x > center_x {
            edge_midpoint_x = center_x;
            edge_midpoint_y = quad.y;
            direction = Directions::TopRight;
        } else if point_x > center_x && point_y == center_y {
            edge_midpoint_x = quad.x + quad.width - 1;
            edge_midpoint_y = center_y;
            direction = Directions::Right;
        } else if point_x < center_x && point_y == center_y {
            edge_midpoint_x = quad.x;
            edge_midpoint_y = center_y;

            direction = Directions::Left;
        }

        match direction {
            Directions::Bottom | Directions::BottomRight | Directions::BottomLeft => {
                let character = self.get(edge_midpoint_x, edge_midpoint_y);
                self.place(
                    edge_midpoint_x,
                    edge_midpoint_y,
                    if character == '─' { '┬' } else { '╤' },
                );
            }

            Directions::Top | Directions::TopRight | Directions::TopLeft => {
                let character = self.get(edge_midpoint_x, edge_midpoint_y);
                self.place(
                    edge_midpoint_x,
                    edge_midpoint_y,
                    if character == '─' { '┴' } else { '╧' },
                );
            }

            Directions::Right => {
                let character = self.get(edge_midpoint_x, edge_midpoint_y);
                self.place(
                    edge_midpoint_x,
                    edge_midpoint_y,
                    if character == '│' { '├' } else { '╟' },
                );
            }

            Directions::Left => {
                let character = self.get(edge_midpoint_x, edge_midpoint_y);
                self.place(
                    edge_midpoint_x,
                    edge_midpoint_y,
                    if character == '│' { '┤' } else { '╢' },
                );
            }

            Directions::None => {}
        }

        match direction {
            Directions::Top => {
                self.draw_vline(edge_midpoint_x, point_y, edge_midpoint_y - point_y, false);
            }

            Directions::Bottom => {
                self.draw_vline(
                    edge_midpoint_x,
                    edge_midpoint_y + 1,
                    point_y - edge_midpoint_y - 1,
                    false,
                );
            }

            Directions::Right => {
                self.draw_hline(
                    edge_midpoint_x + 1,
                    edge_midpoint_y,
                    point_x - edge_midpoint_x - 1,
                    false,
                );
            }

            Directions::Left => {
                self.draw_hline(
                    point_x + 1,
                    edge_midpoint_y,
                    edge_midpoint_x - point_x - 1,
                    false,
                );
            }

            Directions::BottomRight => {
                self.place(line_midpoint_x, line_midpoint_y, '└');
                self.draw_hline(
                    line_midpoint_x + 1,
                    line_midpoint_y,
                    point_x - line_midpoint_x - 1,
                    false,
                );
                self.draw_vline(
                    edge_midpoint_x,
                    edge_midpoint_y + 1,
                    point_y - edge_midpoint_y - 1,
                    false,
                );
            }
            Directions::BottomLeft => {
                self.place(line_midpoint_x, line_midpoint_y, '┘');
                self.draw_hline(point_x, point_y, line_midpoint_x - point_x, false);
                self.draw_vline(
                    edge_midpoint_x,
                    edge_midpoint_y + 1,
                    point_y - edge_midpoint_y - 1,
                    false,
                );
            }
            Directions::TopRight => {
                self.place(line_midpoint_x, line_midpoint_y, '┌');
                self.draw_hline(
                    line_midpoint_x + 1,
                    line_midpoint_y,
                    point_x - line_midpoint_x - 1,
                    false,
                );

                self.draw_vline(
                    edge_midpoint_x,
                    point_y + 1,
                    edge_midpoint_y - point_y - 1,
                    false,
                );
            }
            Directions::TopLeft => {
                self.place(line_midpoint_x, line_midpoint_y, '┐');
                self.draw_hline(point_x, point_y, line_midpoint_x - point_x, false);
                self.draw_vline(
                    edge_midpoint_x,
                    point_y + 1,
                    edge_midpoint_y - point_y - 1,
                    false,
                );
            }
            _ => {}
        }

        self.place(point_x, point_y, '■');
    }
}
