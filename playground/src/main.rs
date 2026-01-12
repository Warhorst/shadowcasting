use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use pad::{p, position::Position};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::{Buffer, Rect},
    style::{Color, Stylize},
    symbols::{Marker, border},
    text::Line,
    widgets::{
        Block, Widget,
        canvas::{Canvas, Painter, Shape},
    },
};
use ratatui_tools::cells::Cells;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}

#[derive(Debug)]
struct App {
    should_exit: bool,
    /// The position from which the shadowcast gets casted
    origin: Position,
    /// All the positions of the walls which block the view
    walls: HashSet<Position>,
    /// All the visible positions calculated by the shadowcast
    visible_positions: HashSet<Position>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = App {
            should_exit: false,
            origin: p!(10, 10),
            walls: HashSet::new(),
            visible_positions: HashSet::new(),
        };
        app.visible_positions = app.shadowcast();
        app
    }
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> std::io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(
        &self,
        frame: &mut Frame,
    ) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => self.should_exit = true,
                    KeyCode::Char('a') => {
                        self.origin.x -= 1;
                        self.visible_positions = self.shadowcast()
                    }
                    KeyCode::Char('d') => {
                        self.origin.x += 1;
                        self.visible_positions = self.shadowcast()
                    }
                    KeyCode::Char('s') => {
                        self.origin.y -= 1;
                        self.visible_positions = self.shadowcast()
                    }
                    KeyCode::Char('w') => {
                        self.origin.y += 1;
                        self.visible_positions = self.shadowcast()
                    }
                    KeyCode::Char(' ') => {
                        if self.walls.contains(&self.origin) {
                            self.walls.remove(&self.origin);
                            self.visible_positions = self.shadowcast();
                        } else {
                            self.walls.insert(self.origin);
                            self.visible_positions = self.shadowcast();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        };

        Ok(())
    }

    fn shadowcast(&self) -> HashSet<Position> {
        shadowcasting::shadow_cast(self.origin, 16, |pos| self.walls.contains(&pos))
    }
}

impl Widget for &App {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let title = Line::from(" Shadowcasting ".bold());
        let instructions = Line::from(vec![
            " Move up".into(),
            " <w>".blue().bold(),
            " Move left".into(),
            " <a>".blue().bold(),
            " Move down".into(),
            " <s>".blue().bold(),
            " Move rigt".into(),
            " <d>".blue().bold(),
            " Toggle wall".into(),
            " <Space>".blue().bold(),
            " Quit".into(),
            " <Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Canvas::default()
            .block(block)
            .x_bounds([0.0, f64::from(area.width)])
            .y_bounds([0.0, f64::from(area.height)])
            .marker(Marker::Block)
            .background_color(Color::Gray)
            .paint(|ctx| {
                let visible = self
                    .visible_positions
                    .iter()
                    .copied()
                    .map(|pos| (pos.x, pos.y))
                    .collect::<Vec<_>>();
                ctx.draw(&Cells::new(&visible, Color::White));

                ctx.draw(&Cells::new(&[(self.origin.x, self.origin.y)], Color::Blue));

                let visible_walls = self
                    .walls
                    .iter()
                    .copied()
                    .filter(|p| self.visible_positions.contains(p))
                    .map(|pos| (pos.x, pos.y))
                    .collect::<Vec<_>>();
                ctx.draw(&Cells::new(&visible_walls, Color::Red));

                let shadowed_walls = self
                    .walls
                    .iter()
                    .copied()
                    .filter(|p| !self.visible_positions.contains(p))
                    .map(|pos| (pos.x, pos.y))
                    .collect::<Vec<_>>();
                ctx.draw(&Cells::new(&shadowed_walls, Color::DarkGray));
            })
            .render(area, buf);
    }
}
