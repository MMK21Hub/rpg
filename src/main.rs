use ratatui::{
    Frame,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal,
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

const UNKNOWN: &'static str = "????";

trait DiscoveredName {
    fn name_as_discovered(&self) -> Option<&'static str>;
}

struct Building {
    name: &'static str,
    name_discovered: bool,
}

impl DiscoveredName for Building {
    fn name_as_discovered(&self) -> Option<&'static str> {
        if self.name_discovered {
            Some(self.name)
        } else {
            None
        }
    }
}

struct Room {
    name: &'static str,
    name_discovered: bool,
}

impl DiscoveredName for Room {
    fn name_as_discovered(&self) -> Option<&'static str> {
        if self.name_discovered {
            Some(self.name)
        } else {
            None
        }
    }
}

struct Game {
    current_building: Building,
    current_room: Room,
}

fn ui(f: &mut Frame, game: &Game) {
    let [header_area, middle_area, text_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(f.area());

    let area_name = Span::styled("Loughborough", Color::Blue);
    let building_name = match game.current_building.name_as_discovered() {
        Some(name) => Span::styled(name, Color::Blue),
        None => Span::styled(UNKNOWN, Color::DarkGray),
    };
    let room_name = match game.current_room.name_as_discovered() {
        Some(name) => Span::styled(name, Color::Blue),
        None => Span::styled(UNKNOWN, Color::DarkGray),
    };

    let label = |text: &'static str| Span::styled(text, Color::DarkGray);
    let header = Paragraph::new(vec![
        Line::from(vec![label("Place: "), area_name]),
        Line::from(vec![label("Building: "), building_name]),
        Line::from(vec![label("Room: "), room_name]),
    ]);

    f.render_widget(header, header_area);

    let room_style = Style::new().add_modifier(Modifier::UNDERLINED);
    let room_name = match game.current_room.name_as_discovered() {
        Some(name) => Span::styled(name, room_style.fg(Color::Blue)),
        None => Span::styled("a room", room_style.fg(Color::DarkGray)),
    };
    let room_text = Line::from(vec![
        Span::styled("> ", Color::DarkGray),
        Span::raw("You find yourself in "),
        room_name,
        Span::raw("."),
    ]);

    let paragraph = Paragraph::new(room_text);

    f.render_widget(paragraph, text_area);
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut terminal = ratatui::init();

    let wavy_top = Building {
        name: "Wavy Top",
        name_discovered: false,
    };
    let room = Room {
        name: "WAV068",
        name_discovered: false,
    };

    let game = Game {
        current_building: wavy_top,
        current_room: room,
    };

    loop {
        terminal.draw(|f| ui(f, &game))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
