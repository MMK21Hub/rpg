use std::collections::HashMap;

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
use serde::{Deserialize, Serialize};

const UNKNOWN: &'static str = "????";

trait DiscoveredName {
    fn name_as_discovered(&self) -> Option<&'static str>;
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum BuildingId {
    WavyTop,
}

impl BuildingId {
    fn data(&self) -> &BuildingData {
        match self {
            BuildingId::WavyTop => &BuildingData { name: "Wavy Top" },
        }
    }
    fn name_as_discovered(&self, state: &GameState) -> Option<&'static str> {
        // TODO use game state
        match self {
            BuildingId::WavyTop => None,
            _ => Some(self.data().name),
        }
    }
}

struct BuildingData {
    name: &'static str,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BuildingState {
    name_discovered: bool,
}

// impl DiscoveredName for Building {
//     fn name_as_discovered(&self) -> Option<&'static str> {
//         if self.name_discovered {
//             Some(self.name)
//         } else {
//             None
//         }
//     }
// }

#[derive(Clone, Copy, Serialize, Deserialize)]
enum RoomId {
    WAV068,
}

impl RoomId {
    fn data(&self) -> &RoomData {
        match self {
            RoomId::WAV068 => &RoomData { name: "WAV068" },
        }
    }
    fn name_as_discovered(&self, state: &GameState) -> Option<&'static str> {
        // TODO use game state
        match self {
            RoomId::WAV068 => None,
            _ => Some(self.data().name),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RoomData {
    name: &'static str,
}

#[derive(Serialize, Deserialize)]
enum Message {
    EnteredRoom(RoomId),
}

#[derive(Serialize, Deserialize)]
struct GameState {
    current_building: BuildingId,
    current_room: RoomId,
    messages: Vec<Message>,
}

impl GameState {
    fn enter_room(&mut self, room: RoomId) {
        self.current_room = room;
        self.messages.push(Message::EnteredRoom(room));
    }
}

struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState {
                current_building: BuildingId::WavyTop,
                current_room: RoomId::WAV068,
                messages: vec![],
            },
        }
    }

    fn current_building(&self) -> &BuildingData {
        &self.state.current_building.data()
    }
    fn current_room(&self) -> &RoomData {
        &self.state.current_room.data()
    }
}

fn ui(f: &mut Frame, game: &Game) {
    let message_area_height: u16 = game.state.messages.len().try_into().unwrap_or(u16::MAX);
    let [header_area, middle_area, message_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(message_area_height),
        ])
        .areas(f.area());

    let area_name = Span::styled("Loughborough", Color::Blue);
    let building_name = match game.state.current_building.name_as_discovered(&game.state) {
        Some(name) => Span::styled(name, Color::Blue),
        None => Span::styled(UNKNOWN, Color::DarkGray),
    };
    let room_name = match game.state.current_room.name_as_discovered(&game.state) {
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
    let room_name = match game.state.current_room.name_as_discovered(&game.state) {
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

    f.render_widget(paragraph, message_area);
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut terminal = ratatui::init();

    let mut game = Game::new();

    game.state.enter_room(RoomId::WAV068);

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
