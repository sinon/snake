use std::time::Duration;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use snake::{SnakeDirection, SnakeGrid};

#[derive(Debug, Default)]
pub struct App {
    grid: SnakeGrid,
    tick_rate: Duration,
    exit: bool,
    height: usize,
    width: usize,
}

impl App {
    pub fn new(height: usize, width: usize) -> Self {
        let grid = SnakeGrid::new_empty(width, height);
        App {
            grid,
            tick_rate: Duration::from_millis(50),
            exit: false,
            height,
            width,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        execute!(std::io::stdout(), EnableMouseCapture)?;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(terminal.get_frame().area())?;
            self.move_snake();
        }
        execute!(std::io::stdout(), DisableMouseCapture)?;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, area: Rect) -> color_eyre::Result<()> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.change_direction(SnakeDirection::Left),
            KeyCode::Up => self.change_direction(SnakeDirection::Up),
            KeyCode::Down => self.change_direction(SnakeDirection::Down),
            KeyCode::Right => self.change_direction(SnakeDirection::Right),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    fn change_direction(&mut self, dir: SnakeDirection) {
        self.grid.change_direction(dir);
    }
    fn move_snake(&mut self) {
        self.grid.move_snake();
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Snake ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Snake Direction ".into(),
            format!("{:?}", self.grid.snake.direction).red().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let grid_out = self.grid.grid.to_string();
        let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
        let grid_text = Text::from(lines);

        Paragraph::new(grid_text).block(block).render(area, buf);
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let s = terminal.size()?;
    let app_result = App::new(s.height as usize, s.width as usize).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 100, 4));

        app.render(buf.area, &mut buf);
        let mut expected = Buffer::with_lines(vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snake ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
        "┃💀💀💀💀💀💀💀💀💀💀                                                                              ┃",
        "┃💀💀💀💀💀💀💀💀💀💀                                                                              ┃",
        "┗━━━━━━━━━ Quit <Q>   ━━━━━━━━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().red().bold();
        let key_style = Style::new().blue().bold();
        // Snake
        expected.set_style(Rect::new(43, 0, 14, 1), title_style);
        // <Q>
        expected.set_style(Rect::new(16, 3, 4, 1), key_style);
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {}
}
