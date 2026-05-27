use ratatui::{
    Frame,
    crossterm::{
        event::{self, Event, KeyCode, KeyModifiers},
        terminal,
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use serde::{Deserialize, Serialize};

const UNKNOWN: &'static str = "????";

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
        if state.discovered_buildings.contains(self) {
            Some(self.data().name)
        } else {
            None
        }
    }
}

struct BuildingData {
    name: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        if state.discovered_rooms.contains(self) {
            Some(self.data().name)
        } else {
            None
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
    WokState(bool),
    Debug(String),
    Log(String),
}

#[derive(Serialize, Deserialize)]
struct GameState {
    current_building: BuildingId,
    current_room: RoomId,
    discovered_buildings: Vec<BuildingId>,
    discovered_rooms: Vec<RoomId>,
    messages: Vec<Message>,
    show_debug_msgs: bool,
    wok: bool,
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
                discovered_buildings: vec![],
                discovered_rooms: vec![],
                messages: vec![],
                wok: false, // Wok Off by default
                show_debug_msgs: false,
            },
        }
    }

    fn push_msg(&mut self, message: Message) {
        self.state.messages.push(message);
    }

    fn current_building(&self) -> &BuildingData {
        &self.state.current_building.data()
    }
    fn current_room(&self) -> &RoomData {
        &self.state.current_room.data()
    }
}

fn stringify_message<'a>(message: &'a Message, game_state: &'a GameState) -> Option<Line<'a>> {
    match message {
        Message::EnteredRoom(room) => {
            let room_style = Style::new().add_modifier(Modifier::UNDERLINED);
            let room_name = match room.name_as_discovered(game_state) {
                Some(name) => Span::styled(name, room_style.fg(Color::Blue)),
                None => Span::styled("a room", room_style.fg(Color::DarkGray)),
            };
            let room_text = Line::from(vec![
                Span::raw("You find yourself in "),
                room_name,
                Span::raw("."),
            ]);
            Some(room_text)
        }
        Message::WokState(on) => Some(Line::from(vec![
            Span::raw("Wok "),
            Span::styled(if *on { "On" } else { "Off" }, Modifier::BOLD),
        ])),
        Message::Debug(text) => {
            if game_state.show_debug_msgs {
                Some(Line::styled(text, Color::DarkGray))
            } else {
                None
            }
        }
        Message::Log(text) => Some(Line::raw(text)),
    }
}

fn ui(f: &mut Frame, game: &Game) {
    let [header_area, middle_area, message_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Min(1),
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

    // Render the lines bottom-aligned in chronological order, up to message_area.height lines
    let message_lines: Vec<Line> = game
        .state
        .messages
        .iter()
        .filter_map(|msg| stringify_message(msg, &game.state))
        .rev()
        .take(message_area.height as usize)
        .collect();
    let mut lines_to_render: Vec<Line> = (0..message_area.height)
        .map(|i| {
            if let Some(line) = message_lines.get(i as usize) {
                line.clone()
            } else {
                Line::default()
            }
        })
        .collect();
    lines_to_render.reverse();
    let messages_paragraph = Paragraph::new(lines_to_render);

    f.render_widget(&messages_paragraph, message_area);
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut terminal = ratatui::init();

    // Let's get gayming
    let mut game = Game::new();
    game.state.enter_room(RoomId::WAV068);

    loop {
        terminal.draw(|f| ui(f, &game))?;
        if let Event::Key(key) = event::read()? {
            game.push_msg(Message::Debug(format!("{:?}", key)));
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('W') => {
                    game.state.wok = !game.state.wok;
                    game.state.messages.push(Message::WokState(game.state.wok));
                }
                KeyCode::Char('D') => {
                    game.state.show_debug_msgs = !game.state.show_debug_msgs;
                    let verb = if game.state.show_debug_msgs {
                        "Shown"
                    } else {
                        "Hidden"
                    };
                    game.push_msg(Message::Debug(format!("{} debug messages", verb)));
                }
                _ => {}
            }
        }
    }

    ratatui::restore();
    Ok(())
}
