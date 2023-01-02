use std::mem;
use ncurses as nc;

use crate::{error, Color};


/// Represents a single page, which can contain a selector and a guide
#[derive(Clone)]
pub struct Page {
    tag: String,
    colors: Colors,
    keybinds: Vec<Keybind>,
    parent: Option<Box<Page>>,
}
impl Page {

    /// Initialize a new instance of Page
    pub fn new(name: String) -> Page {
        Page {
            tag: name,
            colors: Colors::default(),
            keybinds: Vec::new(),
            parent: None,
        }
    }

    /// Set the keybinds for this page
    pub fn keybinds(mut self, binds: Vec<Keybind>) -> Self {
        self.keybinds = binds;
        self
    }

    /// Create a tree of elements
    pub fn elements(mut self, elements: Vec<(Split, Element)>) -> Self {

        self
    }

    /// Set the child pages for this page TODO
    pub fn children(mut self, children: Vec<Page>) -> Self {
        self
    }
}


/// Splits the page in half
//struct Split<'a> {
//    visible: bool,
//    direction: bool, //true is horizontal, false is vertical
//    ratio: f32,         //what percent of the screen space this split will take up (0.1 means this only takes up 10% of the space)
//    element: Element<'a>,
//    next: Option<Box<Split<'a>>>,
//}

/// Defines different types of splits
#[derive(Clone)]
pub enum Split {
    Horizontal(f32),    //horizontal split
    Vertical(f32),      //vertical split
    None,               //no split
}


/// Allows different elements to be stored as one type
#[derive(Clone)]
pub enum Element {
    selector(Selector),
    text(Text),
}
impl Element {
    pub fn new_selector() -> Element {
        Element::selector(Selector::new())
    }

    pub fn new_text(text: String) -> Element {
        Element::text(Text::new(text))
    }

    pub fn align(self, alignment: Align) -> Self {
        match self {
            Self::selector(s) => {
                Element::selector(s.align(alignment))
            },
            Self::text(t) => {
                Element::text(t.align(alignment))
            },
        }
    }

    pub fn tag(self, tag: String) -> Self {
        match self {
            Self::selector(s) => {
                Element::selector(s.tag(tag))
            },
            Self::text(t) => {
                Element::text(t.tag(tag))
            },
        }
    }

    pub fn colors(self, colors: Colors) -> Self {
        match self {
            Self::selector(s) => {
                Element::selector(s.colors(colors))
            },
            Self::text(t) => {
                Element::text(t.colors(colors))
            },
        }
    }

}


/// Lets the user select from a number of options
#[derive(Clone)]
pub struct Selector {
    tag: Option<String>,
    alignment: Align,
    colors: Colors,
    visible: bool,

    item_names: Vec<String>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action>,
    selected_item: u32,     // the currently selected item
}
impl Selector {
    pub fn new() -> Selector {
        Selector {
            tag: None,
            alignment: Align::Center,
            colors: Colors::inherit(),
            visible: true,
            item_names: Vec::new(),
            item_colors: Vec::new(),
            item_actions: Vec::new(),
            selected_item: 0,
        }
    }

    pub fn align(mut self, alignment: Align) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn colors(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }
    
    pub fn tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Retrieves the printed result TODO
    pub fn print(&self) -> String {
        String::new()
    }
}


/// Shows a block of text
#[derive(Clone)]
pub struct Text {
    tag: Option<String>,
    alignment: Align,
    colors: Colors,
    visible: bool,

    text: String,
    width: u32,
    height: u32,
}
impl Text {

    /// Initialize a new instance of Text
    pub fn new(text: String) -> Text {
        
        // first calculate the height and width of the given string
        let (width, height) = calculate_size(&text);
        Text {
            tag: None,
            alignment: Align::Center,
            colors: Colors::inherit(),
            visible: true,
            text,
            width,
            height,
        }
    }

    pub fn align(mut self, alignment: Align) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn colors(mut self, colors: Colors) -> Self {
        self.colors = colors;
        self
    }
    
    pub fn tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Retrieves the printed result TODO
    pub fn print(&self) -> String {
        self.text.clone()
    }
}



/// Defines the colors of text, specifically in a selector or guide
#[derive(Clone)]
pub struct Colors {
    foreground_default: Color,
    background_default: Color,
    foreground_selected: Color,
    background_selected: Color,
}
impl Colors {
    pub fn default() -> Colors {
        Colors {
            foreground_default: Color::White,
            background_default: Color::Black,
            foreground_selected: Color::Black,
            background_selected: Color::White,
        }
    }

    /// Inherit the colors from the page
    pub fn inherit() -> Colors {
        Colors {
            foreground_default: Color::Inherit,
            background_default: Color::Inherit,
            foreground_selected: Color::Inherit,
            background_selected: Color::Inherit,
        }
    }
}


/// Defines actions that Serpent can do
#[derive(Clone)]
pub enum Action
{
    MoveDown,   // for use with selectors
    MoveUp,     // for use with selectors
    PrevPage,
    Quit,       // quit out of Serpent
    ReturnInt(i32),
    ReturnString(String),
    RunFunction(fn() -> ()),
    Select,     // for use with selectors
    ToPage(Page),
}


/// Alignment for various elements
#[derive(Clone)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    CenterLeft,
    Center,
    CenterRight,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}


/// The return type of Serpent
#[derive(Clone)]
pub enum SerpentResult {
    Exit,
    ReturnInt(i32),
    ReturnStr(String),
    None,
}

/// A small container for default keybinds
#[derive(Clone)]
pub struct Keybind {
    key: i32,
    action: Action,
    description: String,
}
impl Keybind {

}

#[derive(Clone)]
pub struct Keybinds {

}
impl Keybinds {

    /// The basic controls for the main page
    pub fn main<'a>() -> Vec<Keybind> {
        vec![
            Keybind{key: 'h' as i32,   action: Action::Quit,       description: String::from("Exit program")},
            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: String::from("Move down")},
            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: String::from("Move up")},
            Keybind{key: 'l' as i32,   action: Action::Select,     description: String::from("Select item")},
            Keybind{key: 27, /*esc*/   action: Action::Quit,       description: String::from("Exit program")},
            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: String::from("Move down")},
            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: String::from("Move up")},
            Keybind{key: 10, /*enter*/ action: Action::Select,     description: String::from("Select item")},
        ]
    }

    /// The basic controls for any child page
    pub fn default<'a>() -> Vec<Keybind> {
        vec![
            Keybind{key: 'h' as i32,   action: Action::PrevPage,   description: String::from("Previous page")},
            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: String::from("Move down")},
            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: String::from("Move up")},
            Keybind{key: 'l' as i32,   action: Action::Select,     description: String::from("Select item")},
            Keybind{key: 27, /*esc*/   action: Action::PrevPage,   description: String::from("Previous page")},
            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: String::from("Move down")},
            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: String::from("Move up")},
            Keybind{key: 10, /*enter*/ action: Action::Select,     description: String::from("Select item")},
        ]
    }
}



/// Given a &str, return the dimensions of it
fn calculate_size(text: &str) -> (u32, u32) {
    let split = text.split("\n");
    let mut largest: u32 = 0;

    for line in split.clone().into_iter() {
        if line.len() as u32 > largest {
            largest = line.len() as u32;
        }
    }

    return (largest, split.count() as u32);
}

