use std::{collections::HashSet, hash::Hash};

// Unsafe tag counter
static mut tag_counter: HashSet<String> = um();

const fn um() -> HashSet<String> {
    return HashSet::new();
}

/// Represents a single page, which can contain a selector and a guide
pub struct Page {
    tag: String,
    selector: Option<Selector>,
    guide: Option<Guide>,
    texts: Vec<Text>,
    vertical_padding: i32,
    horizontal_padding: i32,
}
impl Page {
    pub fn new(tag: String) -> Page {
        Page {
            tag: tag,
            selector: None,
            guide: None,
            texts: Vec::new(),
            vertical_padding: 0,
            horizontal_padding: 0,
        }
    }

    pub fn text(mut self, text: String, alignment: Align, colors: Option<Colors>, tag: Option<String>) -> Self {
        let ntag = match tag {
            None => generate_tag(String::from("TEXT")),
            Some(t) => t.to_string(),
        };
        self.texts.push(Text::new(text, alignment, colors.unwrap(), ntag));
        self
    }
}

// Lets the user select a number of options
pub struct Selector {
    tag: String,
    alignment: Align,
    default_colors: Colors,

    item_names: Vec<String>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action>,
    selected_item: u32,     // the currently selected item
}

// Shows the user keystrokes and their resultant action
pub struct Guide {
    tag: String,
    alignment: Align,
    default_colors: Colors,

    item_names: Vec<String>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action>,
}

// Shows a block of text
pub struct Text {
    tag: String,
    alignment: Align,
    default_colors: Colors,

    text: String,
    width: u32,
    height: u32,
}
impl Text {

    /// Initialize a new instance of Text
    pub fn new(text: String, alignment: Align, default_colors: Colors, tag: String) -> Text {
        // first calculate the height and width of the given string
        let (width, height) = calculate_size(&text);

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
pub struct Colors {
    foreground_default: i16,
    background_default: i16,
    foreground_selected: i16,
    background_selected: i16,
}

// Defines actions that Serpent can do
pub enum Action
{
    ToPage(Page),
    ReturnInt(i32),
    ReturnString(String),
    RunFunction(fn() -> ()),
    Quit,
}

// Alignment for various elements
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



/// Generate a new tag based off of the given input
fn generate_tag(from: String) -> String {
    let mut counter: u32 = 0;
    let mut tag: String = from.clone();

    let tags: HashSet<String>;

    unsafe {
        tags = tag_counter.clone(); // access the global counter
    }

    // keep incrementing the counter until we find a tag that does not exist yet
    loop {
        let new_tag: String = tag.clone() + &counter.to_string();
        if tags.contains(new_tag.as_str()) {
            tag = new_tag;
            unsafe {
                tag_counter.insert(tag.clone());
            }
            break;
        }
        counter += 1;
    }

    return tag;
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

