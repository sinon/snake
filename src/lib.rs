pub mod grid;
pub mod point;

use std::fmt::{self, Display};

use grid::Grid;
use point::{Point, ORTHO_DIR};

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    pub direction: SnakeDirection,
    current_length: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    Snake,
    Empty,
    Apple,
}
impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Snake => {
                write!(f, "#")?;
            }
            CellState::Empty => {
                write!(f, "-")?;
            }
            CellState::Apple => {
                write!(f, "*")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SnakeGrid {
    pub grid: Grid<CellState>,
    pub snake: Snake,
}

impl Default for SnakeGrid {
    fn default() -> Self {
        SnakeGrid::new_empty(10, 10)
    }
}

impl SnakeGrid {
    pub fn new_empty(width: usize, height: usize) -> Self {
        let mut grid = Grid::new_empty(width, height);
        let snake_head = Point::new((width / 2) as i32, (height / 2) as i32);
        let snake_body = Point::new((width / 2) as i32 + 1, (height / 2) as i32);
        let snake_tail = Point::new((width / 2) as i32 + 2, (height / 2) as i32);
        grid[snake_head] = CellState::Snake;
        grid[snake_body] = CellState::Snake;
        grid[snake_tail] = CellState::Snake;
        let snake = Snake {
            body: vec![snake_head, snake_body, snake_tail],
            direction: SnakeDirection::Right,
            current_length: 3,
        };
        SnakeGrid { grid, snake }
    }

    fn convert_direction_to_next_point(&self, dir: SnakeDirection) -> Point {
        let offset = match dir {
            SnakeDirection::Up => ORTHO_DIR[0],
            SnakeDirection::Down => ORTHO_DIR[1],
            SnakeDirection::Left => ORTHO_DIR[2],
            SnakeDirection::Right => ORTHO_DIR[3],
        };
        self.snake.body[0] + offset
    }

    pub fn change_direction(&mut self, dir: SnakeDirection) {
        let valid_moves: Vec<Point> = self.grid.get_valid_moves(self.snake.body[0]).collect();

        if valid_moves.contains(&self.convert_direction_to_next_point(dir)) {
            self.snake.direction = dir;
        } else {
            // TODO: Handle invalid move
            panic!("Invalid move");
        }
    }
    pub fn move_snake(&mut self) {
        let mut snake = self.snake.body.clone();
        snake.pop();
        let new_head = match self.snake.direction {
            SnakeDirection::Up => snake[0] + ORTHO_DIR[0],
            SnakeDirection::Down => snake[0] + ORTHO_DIR[1],
            SnakeDirection::Left => snake[0] + ORTHO_DIR[2],
            SnakeDirection::Right => snake[0] + ORTHO_DIR[3],
        };
        if !self.grid.contains(&new_head) {
            // TODO: Handle graceful failure
            panic!("Snake out of bounds");
        }

        let mut new_snake = vec![new_head];
        for s in snake.iter() {
            if new_snake.len() < self.snake.current_length + 1 {
                new_snake.push(*s);
            } else {
                break;
            }
        }
        self.grid.cells = self.grid.cells.iter().map(|_| CellState::Empty).collect();
        for p in &new_snake {
            self.grid[*p] = CellState::Snake;
        }
        self.snake.current_length = new_snake.len();
        self.snake.body = new_snake;
    }
}

impl Grid<CellState> {
    pub fn new_empty(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells: Vec<CellState> = (0..size).map(|_| CellState::Empty).collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    fn get_valid_moves(&self, point: Point) -> impl Iterator<Item = Point> + use<'_> {
        // TODO: Should take into consideration if a space is occupied by a snake or not
        ORTHO_DIR
            .into_iter()
            .map(move |d| point + d)
            .filter(|p| self.contains(p))
    }
}

impl Default for Grid<CellState> {
    fn default() -> Self {
        Grid::new_empty(10, 10)
    }
}

impl Display for Grid<CellState> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for w in row * self.width..(row + 1) * self.width {
                write!(f, "{}", self.cells[w])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_try_get() {
        let g = Grid::new_empty(0, 0);
        assert!(g.try_get(Point { x: 10, y: 10 }) == None);
    }

    #[test]
    fn test_grid_display() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Snake;
        let s = format!("{}", g);
        assert_eq!(s, "💀💀💀\n💀😇💀\n💀💀💀\n".to_string());
    }

    #[test]
    fn test_grid_debug() {
        let mut g = Grid::new_empty(3, 3);
        g.cells[4] = CellState::Snake;
        let s = format!("{:?}", g);
        assert_eq!(s, "Grid { width: 3, height: 3, cells: [Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead] }".to_string());
    }
}
