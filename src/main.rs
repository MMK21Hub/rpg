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

const UNKNOWN: &'static str = "????";

trait DiscoveredName {
    fn name_as_discovered(&self) -> Option<&'static str>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct BuildingId(&'static str);
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct RoomId(&'static str);
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

enum Message {
    EnteredRoom(RoomId),
}

struct World {
    buildings: HashMap<BuildingId, Building>,
    rooms: HashMap<RoomId, Room>,
}

impl World {
    fn building(&self, id: BuildingId) -> &Building {
        self.buildings
            .get(&id)
            .unwrap_or_else(|| panic!("Missing BuildingId: {:?}", id))
    }

    fn room(&self, id: RoomId) -> &Room {
        self.rooms
            .get(&id)
            .unwrap_or_else(|| panic!("Missing RoomId: {:?}", id))
    }
}

struct Game {
    current_building: BuildingId,
    current_room: RoomId,
    messages: Vec<Message>,
    world: World,
}

impl Game {
    fn enter_room(&mut self, room: RoomId) {
        self.current_room = room;
        self.messages.push(Message::EnteredRoom(room));
    }
    fn current_building(&self) -> &Building {
        self.world.building(self.current_building)
    }
    fn current_room(&self) -> &Room {
        self.world.room(self.current_room)
    }
}

fn ui(f: &mut Frame, game: &Game) {
    let message_area_height: u16 = game.messages.len().try_into().unwrap_or(u16::MAX);
    let [header_area, middle_area, message_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(message_area_height),
        ])
        .areas(f.area());

    let area_name = Span::styled("Loughborough", Color::Blue);
    let building_name = match game.current_building().name_as_discovered() {
        Some(name) => Span::styled(name, Color::Blue),
        None => Span::styled(UNKNOWN, Color::DarkGray),
    };
    let room_name = match game.current_room().name_as_discovered() {
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
    let room_name = match game.current_room().name_as_discovered() {
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

    let mut game = Game {
        current_building: BuildingId("wavy_top"),
        current_room: RoomId("wav_068"),
        messages: vec![],
        world: World {
            buildings: HashMap::from([(
                BuildingId("wavy_top"),
                Building {
                    name: "Wavy Top",
                    name_discovered: false,
                },
            )]),
            rooms: HashMap::from([(
                RoomId("wav_068"),
                Room {
                    name: "WAV068",
                    name_discovered: false,
                },
            )]),
        },
    };

    game.messages.push(Message::EnteredRoom(game.current_room));

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
