use std::fmt::Display;

const UNKNOWN_NAME: &'static str = "????";

struct Building {
    name: &'static str,
    name_discovered: bool,
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.name_discovered {
            self.name
        } else {
            UNKNOWN_NAME
        };
        write!(f, "{}", name)
    }
}

fn main() {
    let wavy_top = Building {
        name: "Eiffel Tower",
        name_discovered: false,
    };
    println!("Building: {}", wavy_top);
}
