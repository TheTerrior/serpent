use std::{mem, cell::RefCell, rc::Rc};
use ncurses as nc;

use crate::{error, Color, SerpentWriter, SerpentElement};


/// Represents a single page, which can contain a selector and a guide
#[derive(Clone)]
pub struct Page {
    pub partitions: Vec<PartitionInfo>, //allows verification information to be abstracted from a Partition
    //pub keybinds: Vec<Keybind>,
}
impl Page {

    /// Initialize a new instance of Page
    pub fn new() -> Page {
        Page {
            partitions: Vec::new(),
            //keybinds: Keybinds::default(),

        }
    }


    /// Print this page to the screen
    pub fn show(&mut self, output: &mut SerpentWriter) {

        //iterate through all partitions in this page
        for info in &self.partitions {
            let partition = info.partition.clone();
            let offset = info.offset;
            let size = info.size;

            let partition_ref = partition.clone();
            partition_ref.borrow().show(output); //let the writer print its messages to the immutable writer
            Page::print(output, size, offset);
        }
    }


    /// Print a SerpentWriter's contents to the screen, TODO
    fn print(output: &mut SerpentWriter, size: (usize, usize), offset: (usize, usize)) {

    }
}



/// Helper struct for the partitions list in Page
#[derive(Clone)]
pub struct PartitionInfo {
    pub partition: Rc<RefCell<Partition>>,
    pub offset: (usize, usize),
    pub size: (usize, usize),
}
impl PartitionInfo {
    pub fn new(partition: Rc<RefCell<Partition>>, offset: (usize, usize), size: (usize, usize)) -> Self {
        PartitionInfo {partition, offset, size}
    }
}



/// One partition of a page's full area
#[derive(Clone)]
pub struct Partition {
    parent: Rc<RefCell<Page>>,  //a link to the partition's parent
    index: usize, //the index of this partition in the page's list of partitions
    pub element: Option<&'static dyn SerpentElement>,   //the element this partition points to, must implement SerpentElement
}
impl Partition {
    
    /// Initialize a new partition
    pub fn new(parent: Rc<RefCell<Page>>, index: usize) -> Self {
        Partition { parent: parent, index: index, element: None }
    }


    /// Get the size of this partition
    pub fn get_size(&self) -> (usize, usize) {
        self.parent.borrow().partitions[self.index].size
    }


    /// Split this partition into multiple new partitions, TODO
    pub fn split<const N: usize>(mut self) -> [Self; N] {
        let mut ret: [Partition; N] = unsafe {mem::MaybeUninit::uninit().assume_init()}; //initialize empty array

        // for each item in the length
        for i in 0..N {
            ret[i] = Partition::new(self.parent.clone(), 0);
        }
        ret
    }


    /// Call the internal element's show method
    pub fn show(&self, output: &SerpentWriter) {
        if let Some(elem) = self.element {
            elem.show(output);
        }
    }
}




























///// Lets the user select from a number of options
//#[derive(Clone)]
//pub struct Selector {
//    tag: Option<String>,
//    alignment: Align,
//    //colors: Colors,
//    visible: bool,
//
//    item_names: Vec<String>,
//    //item_colors: Vec<Option<Colors>>,   // if set to None, use the default for this selector
//    item_actions: Vec<Action>,
//    selected_item: u32,     // the currently selected item
//}
//impl Selector {
//    pub fn new() -> Selector {
//        Selector {
//            tag: None,
//            alignment: Align::Center,
//            //colors: Colors::inherit(),
//            visible: true,
//            item_names: Vec::new(),
//            //item_colors: Vec::new(),
//            item_actions: Vec::new(),
//            selected_item: 0,
//        }
//    }
//
//    pub fn align(mut self, alignment: Align) -> Self {
//        self.alignment = alignment;
//        self
//    }
//
//
//    pub fn tag(mut self, tag: String) -> Self {
//        self.tag = Some(tag);
//        self
//    }
//
//    pub fn visible(mut self, visible: bool) -> Self {
//        self.visible = visible;
//        self
//    }
//
//    /// Retrieves the printed result TODO
//    pub fn print(&self) -> String {
//        String::new()
//    }
//}
//
//
///// Shows a block of text
//#[derive(Clone)]
//pub struct Text {
//    tag: Option<String>,
//    alignment: Align,
//    //colors: Colors,
//    visible: bool,
//
//    text: String,
//    width: u32,
//    height: u32,
//}
//impl Text {
//
//    /// Initialize a new instance of Text
//    pub fn new(text: String) -> Text {
//        
//        // first calculate the height and width of the given string
//        let (width, height) = calculate_size(&text);
//        Text {
//            tag: None,
//            alignment: Align::Center,
//            //colors: Colors::inherit(),
//            visible: true,
//            text,
//            width,
//            height,
//        }
//    }
//
//    pub fn align(mut self, alignment: Align) -> Self {
//        self.alignment = alignment;
//        self
//    }
//
//    
//    pub fn tag(mut self, tag: String) -> Self {
//        self.tag = Some(tag);
//        self
//    }
//
//    pub fn visible(mut self, visible: bool) -> Self {
//        self.visible = visible;
//        self
//    }
//
//    /// Retrieves the printed result TODO
//    pub fn print(&self) -> String {
//        self.text.clone()
//    }
//}
//
//
//
//
//
///// Defines actions that Serpent can do
//#[derive(Clone)]
//pub enum Action
//{
//    MoveDown,   // for use with selectors
//    MoveUp,     // for use with selectors
//    PrevPage,
//    Quit,       // quit out of Serpent
//    ReturnInt(i32),
//    ReturnString(String),
//    RunFunction(fn() -> ()),
//    Select,     // for use with selectors
//    ToPage(Page),
//}
//
//
///// Alignment for various elements
//#[derive(Clone)]
//pub enum Align {
//    TopLeft,
//    Top,
//    TopRight,
//    Left,
//    CenterLeft,
//    Center,
//    CenterRight,
//    Right,
//    BottomLeft,
//    Bottom,
//    BottomRight,
//}
//
//
///// The return type of Serpent
//#[derive(Clone)]
//pub enum SerpentResult {
//    Exit,
//    ReturnInt(i32),
//    ReturnStr(String),
//    None,
//}
//
///// A small container for default keybinds
//#[derive(Clone)]
//pub struct Keybind {
//    key: i32,
//    action: Action,
//    description: String,
//}
//impl Keybind {
//
//}
//
//#[derive(Clone)]
//pub struct Keybinds {
//
//}
//impl Keybinds {
//
//    /// The basic controls for the main page
//    pub fn main<'a>() -> Vec<Keybind> {
//        vec![
//            Keybind{key: 'h' as i32,   action: Action::Quit,       description: String::from("Exit program")},
//            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: String::from("Move down")},
//            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: String::from("Move up")},
//            Keybind{key: 'l' as i32,   action: Action::Select,     description: String::from("Select item")},
//            Keybind{key: 27, /*esc*/   action: Action::Quit,       description: String::from("Exit program")},
//            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: String::from("Move down")},
//            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: String::from("Move up")},
//            Keybind{key: 10, /*enter*/ action: Action::Select,     description: String::from("Select item")},
//        ]
//    }
//
//    /// The basic controls for any child page
//    pub fn default<'a>() -> Vec<Keybind> {
//        vec![
//            Keybind{key: 'h' as i32,   action: Action::PrevPage,   description: String::from("Previous page")},
//            Keybind{key: 'j' as i32,   action: Action::MoveDown,   description: String::from("Move down")},
//            Keybind{key: 'k' as i32,   action: Action::MoveUp,     description: String::from("Move up")},
//            Keybind{key: 'l' as i32,   action: Action::Select,     description: String::from("Select item")},
//            Keybind{key: 27, /*esc*/   action: Action::PrevPage,   description: String::from("Previous page")},
//            Keybind{key: 66, /*down*/  action: Action::MoveDown,   description: String::from("Move down")},
//            Keybind{key: 65, /*up*/    action: Action::MoveUp,     description: String::from("Move up")},
//            Keybind{key: 10, /*enter*/ action: Action::Select,     description: String::from("Select item")},
//        ]
//    }
//}



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

