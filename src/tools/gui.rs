use crate::traits::command::Command;

pub struct GUI {}

const TITLE_SPACING: u32 = 2;
const SUB_TITLE_SPACING: u32 = 4;
const CONTENT_SPACING: u32 = 6;

impl GUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_params(&self, command: &dyn Command) {
        if command.get_params().len() > 0 {
            self.sub_title("params:");

            for param in command.get_params().iter() {
                let spacing: String = (0..(CONTENT_SPACING + 10 - param.0.len() as u32))
                    .map(|_| ' ')
                    .collect();

                self.content(&format!(
                    "{key}:{spacing}{value}",
                    key = param.0,
                    spacing = spacing,
                    value = param.1
                ));
            }

            self.nl();
        }
    }

    pub fn title(&self, title: &str) -> &Self {
        self.nl();
        let spacing: String = (0..TITLE_SPACING).map(|_| ' ').collect();
        println!("{}## {} ##\n", spacing, title);
        self
    }

    pub fn sub_title(&self, title: &str) -> &Self {
        let spacing: String = (0..SUB_TITLE_SPACING).map(|_| ' ').collect();
        println!("{}{}", spacing, title);
        self
    }

    pub fn content(&self, title: &str) -> &Self {
        let spacing: String = (0..CONTENT_SPACING).map(|_| ' ').collect();
        println!("{}{}", spacing, title);
        self
    }

    pub fn nl(&self) -> &Self {
        println!();
        self
    }
}
