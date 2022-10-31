
// Represents a single page, which can contain a selector and a guide
struct Page<'a> {
    name: &'a str,
    selector: Option<Selector<'a>>,
    guide: Option<Guide<'a>>,
}

// Lets the user select a number of options
struct Selector<'a> {
    name: &'a str,
    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
    default_colors: Colors,

    selected_item: i32,     // the currently selected item
}

// Shows the user keystrokes and their resultant action
struct Guide<'a> {
    name: &'a str,
    item_names: Vec<&'a str>,
    item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
    item_actions: Vec<Action<'a>>,
    default_colors: Colors,
}

// Defines the colors of text, specifically in a selector or guide
struct Colors {
    foreground_default: i16,
    background_default: i16,
    foreground_selected: i16,
    background_selected: i16,
}

// Defines actions that Serpent can do
enum Action<'a> {
    ToPage(i32),
    ReturnInt(i32),
    ReturnString(&'a str),
    RunFunction(),
    Quit,
}



