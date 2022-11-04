use ncurses as nc;
use crate::color;
use std::{collections::HashSet, hash::Hash};


/// Represents a single page, which can contain a selector and a guide
pub struct Page<'a> {
    tag: &'a str,
    default_colors: Colors,

    keybinds: Vec<(i32, Action<'a>, &'a str)>,
    selector: Option<Selector<'a>>, // allows the user to select an option on-screen
    guide: Option<Guide<'a>>,       // allows the user to type a certain character for an action
    texts: Vec<Text<'a>>,
    vertical_padding: i32,
    horizontal_padding: i32,

    parent: Option<&'a Page<'a>>,
}
impl<'a> Page<'a> {

    /// Initialize a new instance of Page
    pub fn new(tag: &'a str) -> Page {
        Page {
            tag: tag,
            default_colors: Colors{
                foreground_default: nc::COLOR_WHITE,
                background_default: nc::COLOR_BLACK,
                foreground_selected: nc::COLOR_BLACK,
                background_selected: nc::COLOR_WHITE,
            },
            keybinds: Vec::new(),
            selector: None,
            guide: None,
            texts: Vec::new(),
            vertical_padding: 0,
            horizontal_padding: 0,
            parent: None,
        }
    }

    /// Create a new text element for this page<br>
    /// Make sure to provide a unique tag (identifier)
    pub fn text(mut self, text: &'a str, alignment: Align, colors: Option<Colors>, tag: &'a str) -> Self {
        let ncolors = match colors {
            None => self.default_colors.clone(),
            Some(nc) => nc,
        };
        self.texts.push(Text::new(text, alignment, ncolors, tag));
        self
    }

    /// Set the keybinds for this page<br>
    pub fn keybinds(mut self, controls: Vec<(i32, Action<'a>, &'a str)>) -> Self {
        self.keybinds = controls;
        self
    }

    /// Set the child pages for this page
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

pub enum Split {
    Horizontal(f32),    //horizontal split
    Vertical(f32),      //vertical split
    None,               //no split
}


/// Allows different elements to be stored as one type
enum Element<'a> {
    selector(Selector<'a>),
    guide(Guide<'a>),
    text(Text<'a>),
}


/// Lets the user select a number of options
pub struct Selector<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,
    visible: bool,

    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
    selected_item: u32,     // the currently selected item
}


/// Shows the user keystrokes and their resultant action
pub struct Guide<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,
    visible: bool,
}


/// Shows a block of text
#[derive(Clone)]
pub struct Text<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,
    visible: bool,

    text: &'a str,
    width: u32,
    height: u32,
}
impl<'a> Text<'a> {

    /// Initialize a new instance of Text
    pub fn new(text: &'a str, alignment: Align, default_colors: Colors, tag: &'a str) -> Text<'a> {
        // first calculate the height and width of the given string
        let (width, height) = calculate_size(text);

        Text {
            tag,
            alignment,
            default_colors,
            visible: true,
            text,
            width,
            height,
        }
    }

    /// Retrieves the printed result
    pub fn print(&self) -> String {
        String::from(self.text)
    }
}



/// Defines the colors of text, specifically in a selector or guide
#[derive(Clone)]
pub struct Colors {
    foreground_default: i16,
    background_default: i16,
    foreground_selected: i16,
    background_selected: i16,
}
impl Colors {
    pub fn default() -> Colors {
        Colors {
            foreground_default: color::WHITE,
            background_default: color::BLACK,
            foreground_selected: color::BLACK,
            background_selected: color::WHITE,
        }
    }
}


/// Defines actions that Serpent can do
pub enum Action<'a>
{
    MoveDown,   // for use with selectors
    MoveUp,     // for use with selectors
    PrevPage,
    Quit,       // quit out of Serpent
    ReturnInt(i32),
    ReturnString(String),
    RunFunction(fn() -> ()),
    Select,     // for use with selectors
    ToPage(Page<'a>),
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
pub enum SerpentResult {
    Exit,
    ReturnInt(i32),
    ReturnStr(String),
    None,
}

/// A small container for default keybinds
pub struct Keybinds {

}
impl Keybinds {

    /// The basic controls for the main page
    pub fn main<'a>() -> Vec<(i32, Action<'a>, &'a str)> {
        vec![
            ('h' as i32,    Action::Quit,       "Exit program"),
            ('j' as i32,    Action::MoveDown,   "Move down"),
            ('k' as i32,    Action::MoveUp,     "Move up"),
            ('l' as i32,    Action::Select,     "Select item"),
            (27, /*esc*/    Action::Quit,       "Exit program"),
            (66, /*down*/   Action::MoveDown,   "Move down"),
            (65, /*up*/     Action::MoveUp,     "Move up"),
            (10, /*enter*/  Action::Select,     "Select item"),
        ]
    }

    /// The basic controls for any child page
    pub fn default<'a>() -> Vec<(i32, Action<'a>, &'a str)> {
        vec![
            ('h' as i32,    Action::PrevPage,   "Previous page"),
            ('j' as i32,    Action::MoveDown,   "Move down"),
            ('k' as i32,    Action::MoveUp,     "Move up"),
            ('l' as i32,    Action::Select,     "Select item"),
            (27, /*esc*/    Action::PrevPage,   "Previous page"),
            (66, /*down*/   Action::MoveDown,   "Move down"),
            (65, /*up*/     Action::MoveUp,     "Move up"),
            (10, /*enter*/  Action::Select,     "Select item"),
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

