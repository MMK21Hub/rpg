use colored::Colorize;

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

fn main() {
    let wavy_top = Building {
        name: "Wavy Top",
        name_discovered: false,
    };
    let room = Room {
        name: "WAV068",
        name_discovered: false,
    };

    draw_header(&wavy_top, &room);

    println!();
    print_entered_room(&room);
}

fn print_entered_room(room: &Room) {
    let room_name = match room.name_as_discovered() {
        Some(name) => name.underline(),
        None => "a room".bright_black().underline(),
    };
    println!("You find yourself in {}.", room_name);
}

fn draw_header(building: &Building, room: &Room) {
    println!("{} {}", "Place:".bright_black(), "Loughborough".blue());
    println!(
        "{} {}",
        "Building:".bright_black(),
        match building.name_as_discovered() {
            Some(name) => name.blue(),
            None => UNKNOWN.bright_black(),
        }
    );
    println!(
        "{} {}",
        "Room:".bright_black(),
        match room.name_as_discovered() {
            Some(name) => name.blue(),
            None => UNKNOWN.bright_black(),
        }
    );
}
