use ncurses as nc;

use crate::color;
use crate::error;


/// Represents a single page, which can contain a selector and a guide
pub struct Page<'a> {
    tag: &'a str,
    colors: Colors,
    keybinds: Vec<Keybind<'a>>,
    parent: Option<&'a Page<'a>>,
}
impl<'a> Page<'a> {

    /// Initialize a new instance of Page
    pub fn new(name: &'a str) -> Page {
        Page {
            tag: name,
            colors: Colors::default(),
            keybinds: Vec::new(),
            parent: None,
        }
    }

    /// Set the keybinds for this page
    pub fn keybinds(mut self, binds: Vec<Keybind<'a>>) -> Self {
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
pub enum Split {
    Horizontal(f32),    //horizontal split
    Vertical(f32),      //vertical split
    None,               //no split
}


/// Allows different elements to be stored as one type
pub enum Element<'a> {
    selector(Selector<'a>),
    guide(Guide<'a>),
    text(Text<'a>),
}
impl<'a> Element<'a> {
    pub fn new_selector() -> Element<'a> {
        Element::selector(Selector::new())
    }

    pub fn new_guide() -> Element<'a> {
        Element::guide(Guide::new())
    }

    pub fn new_text(text: &'a str) -> Element<'a> {
        Element::text(Text::new(text))
    }

}


/// Lets the user select from a number of options
pub struct Selector<'a> {
    tag: Option<&'a str>,
    alignment: Align,
    colors: Colors,
    visible: bool,

    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
    selected_item: u32,     // the currently selected item
}
impl<'a> Selector<'a> {
    pub fn new() -> Selector<'a> {
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
    
    pub fn tag(mut self, tag: &'a str) -> Self {
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


/// Shows the user keystrokes for this page and their resultant actions
pub struct Guide<'a> {
    tag: Option<&'a str>,
    alignment: Align,
    colors: Colors,
    visible: bool,
}
impl<'a> Guide<'a> {
    pub fn new() -> Guide<'a> {
        Guide {
            tag: None,
            alignment: Align::Center,
            colors: Colors::inherit(),
            visible: true,
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
    
    pub fn tag(mut self, tag: &'a str) -> Self {
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
pub struct Text<'a> {
    tag: Option<&'a str>,
    alignment: Align,
    colors: Colors,
    visible: bool,

    text: &'a str,
    width: u32,
    height: u32,
}
impl<'a> Text<'a> {

    /// Initialize a new instance of Text
    pub fn new(text: &'a str) -> Text<'a> {
        
        // first calculate the height and width of the given string
        let (width, height) = calculate_size(text);
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
    
    pub fn tag(mut self, tag: &'a str) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Retrieves the printed result TODO
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

    /// Inherit the colors from the page
    pub fn inherit() -> Colors {
        Colors {
            foreground_default: color::INHERIT,
            background_default: color::INHERIT,
            foreground_selected: color::INHERIT,
            background_selected: color::INHERIT,
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
pub struct Keybind<'a> {
    key: i32,
    action: Action<'a>,
    description: &'a str,
}
impl<'a> Keybind<'a> {

}

pub struct Keybinds {

}
impl Keybinds {

    /// The basic controls for the main page
    pub fn main<'a>() -> Vec<Keybind<'a>> {
        vec![
            Keybind{key: 'h' as i32,   action: Action::Quit,       description: "Exit program"},
            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: "Move down"},
            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: "Move up"},
            Keybind{key: 'l' as i32,   action: Action::Select,     description: "Select item"},
            Keybind{key: 27, /*esc*/   action: Action::Quit,       description: "Exit program"},
            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: "Move down"},
            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: "Move up"},
            Keybind{key: 10, /*enter*/ action: Action::Select,     description: "Select item"},
        ]
    }

    /// The basic controls for any child page
    pub fn default<'a>() -> Vec<Keybind<'a>> {
        vec![
            Keybind{key: 'h' as i32,   action: Action::PrevPage,   description: "Previous page"},
            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: "Move down"},
            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: "Move up"},
            Keybind{key: 'l' as i32,   action: Action::Select,     description: "Select item"},
            Keybind{key: 27, /*esc*/   action: Action::PrevPage,   description: "Previous page"},
            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: "Move down"},
            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: "Move up"},
            Keybind{key: 10, /*enter*/ action: Action::Select,     description: "Select item"},
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

