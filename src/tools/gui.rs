pub struct GUI {}

const TITLE_SPACING: u32 = 2;
const SUB_TITLE_SPACING: u32 = 4;
const CONTENT_SPACING: u32 = 6;

impl GUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self, title: &str) -> &Self {
        self.nl();
        let spacing: String = (0..TITLE_SPACING).map(|_| ' ').collect();
        println!("{}## {} ##", spacing, title);
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
