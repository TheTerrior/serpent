use ncurses as nc;
use std::{collections::HashSet, hash::Hash};

/// Represents a single page, which can contain a selector and a guide
pub struct Page<'a> {
    tag: &'a str,
    selector: Option<Selector<'a>>,
    guide: Option<Guide<'a>>,
    texts: Vec<Text<'a>>,
    vertical_padding: i32,
    horizontal_padding: i32,
    default_colors: Colors,
}
impl<'a> Page<'a> {

    /// Initialize a new instance of Page
    pub fn new(tag: &'a str) -> Page {
        Page {
            tag: tag,
            selector: None,
            guide: None,
            texts: Vec::new(),
            vertical_padding: 0,
            horizontal_padding: 0,
            default_colors: Colors{
                foreground_default: nc::COLOR_WHITE,
                background_default: nc::COLOR_BLACK,
                foreground_selected: nc::COLOR_BLACK,
                background_selected: nc::COLOR_WHITE,
            }
        }
    }

    /// Create a new text element for this page<br>
    /// Please make sure to provide a unique tag (identifier)
    pub fn text(mut self, text: &'a str, alignment: Align, colors: Option<Colors>, tag: &'a str) -> Self {
        let ncolors = match colors {
            None => self.default_colors.clone(),
            Some(nc) => nc,
        };
        self.texts.push(Text::new(text, alignment, ncolors, tag));
        self
    }
}

// Lets the user select a number of options
pub struct Selector<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,

    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
    selected_item: u32,     // the currently selected item
}

// Shows the user keystrokes and their resultant action
pub struct Guide<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,

    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
}

// Shows a block of text
#[derive(Clone)]
pub struct Text<'a> {
    tag: &'a str,
    alignment: Align,
    default_colors: Colors,

    text: &'a str,
    width: u32,
    height: u32,
}
impl<'a> Text<'a> {

    /// Initialize a new instance of Text
    pub fn new(text: &'a str, alignment: Align, default_colors: Colors, tag: &'a str) -> Text {
        // first calculate the height and width of the given string
        let (width, height) = calculate_size(text);

        Text {
            tag,
            alignment,
            default_colors,
            text,
            width,
            height,
        }
    }
}

// Defines the colors of text, specifically in a selector or guide
#[derive(Clone)]
pub struct Colors {
    foreground_default: i16,
    background_default: i16,
    foreground_selected: i16,
    background_selected: i16,
}

// Defines actions that Serpent can do
pub enum Action<'a>
{
    ToPage(Page<'a>),
    ReturnInt(i32),
    ReturnString(&'a str),
    RunFunction(fn() -> ()),
    Quit,
}

// Alignment for various elements
#[derive(Clone)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

// The return type of Serpent
pub enum SerpentResult {
    ReturnInt(i32),
    ReturnStr(String),
    None,
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

