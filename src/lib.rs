pub mod internal;
pub mod error;
pub mod elements;
pub mod constants;

use std::{cell::RefCell, rc::Rc, collections::{HashMap, HashSet}, hash::Hash};

use internal::*;
use error::SerpentError;


//
/// SERPENT INTERFACE
//

/// Start ncurses
pub fn start() {
    ncurses::initscr();
    ncurses::noecho();
}

/// Stop ncurses
pub fn stop() {
    ncurses::endwin();
}

/// Refresh ncurses
pub fn refresh() {
    ncurses::refresh();
}

/// Will relaunch ncurses, whether it's currently running or not
pub fn restart() {
    ncurses::endwin();
    ncurses::refresh();
    ncurses::initscr();
}

/// Create a new instance of UI
pub fn new() -> UI {
    //UI::new(ncurses::initscr()) //start ncurses and save the window to the UI struct
    ncurses::initscr();
    UI::new() //start ncurses
}



//
/// UI IMPLEMENTATION
//

/// Main controller for Serpent, utilizes ncurses
pub struct UI {
    pages: Vec<Rc<RefCell<Page>>>,
    action_counter: u32,
    actions: HashMap<i32, HashSet<u32>>, //binds a phyiscal key to an action
}
impl UI {

    /// Create a new instance of UI
    fn new() -> UI {
        ncurses::intrflush(ncurses::stdscr(), false); //not sure what this does
        ncurses::keypad(ncurses::stdscr(), true); //enable keypad, so that arrow keys and other buttons function
        ncurses::clear(); //not sure
        ncurses::noecho(); //don't print whatever is typed
        ncurses::mousemask(ncurses::ALL_MOUSE_EVENTS as ncurses::mmask_t, None); //enable mouse events
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE); //make mouse invisible
        UI {
            pages: Vec::new(),
            action_counter: 0,
            actions: HashMap::new(),
        }
    }

    /// Create a new page in this UI, returns a page index, the partition's size, and the page's base partition
    pub fn new_page(&mut self) -> Result<(usize, Rc<RefCell<Partition>>), SerpentError> {
        let page_ref = Rc::new(RefCell::new(Page::new())); //place a new page into an rc, set its index
        let t_size = termsize::get() //get the size of the terminal in rows and columns
            .map(|size| (size.cols as usize, size.rows as usize))
            .ok_or(SerpentError::TerminalSizeError)?;

        // create the base partition for this new page
        let base_partition = Partition::new(page_ref.clone(), 0);
        let base_partition_ref = Rc::new(RefCell::new(base_partition)); //place the base partition into an rc

        // push the base partition to the page's list of partitions
        page_ref.borrow_mut().partitions = vec![PartitionInfo::new(base_partition_ref.clone(), (0, 0), t_size)];

        self.pages.push(page_ref); //push the page to the UI's list of pages
        Ok((self.pages.len()-1, base_partition_ref)) //return an rc to the partition
    }


    /// Gets user input and returns the result
    pub fn next(&self) -> SerpentEvent {
        ncurses::refresh(); //refresh the screen
        let event = ncurses::getch();
        if event == ncurses::KEY_MOUSE { //if received a mouse event
            let mut mevent: ncurses::MEVENT = unsafe {std::mem::MaybeUninit::uninit().assume_init()};
            ncurses::getmouse(&mut mevent);
            SerpentEvent {
                input: InputType::Mouse{
                    x: mevent.x,
                    y: mevent.y,
                    event: mevent.bstate,
                },
                actions: Vec::new(),
            }
        } else { //if a keyboard event
            SerpentEvent {
                input: InputType::Key(event),
                actions: Vec::new(),
            }
        }
    }


    /// Creates a new action for this UI
    pub fn new_action(&mut self) -> u32 {
        self.action_counter += 1;
        self.action_counter - 1
    }


    /// Binds a key to an action globally, returns whether the binding was present
    pub fn bind_global(&mut self, key: i32, action: u32) -> bool {
        let res = self.actions.get_mut(&key);
        match res {
            None => { //key is not bound to anything
                self.actions.insert(key, HashSet::from([action])) != None
            },
            Some(set) => { //key is bound to something
                !set.insert(action)
            },
        }
    }


    /// Unbinds a key from an action globally, returns whether the binding was present
    pub fn unbind_global(&mut self, key: i32, action: u32) -> bool {
        let res = self.actions.get_mut(&key);
        if let Some(set) = res { //key is bound to the action
            set.remove(&action)
        } else { //key not bound to the action
            false
        }
    }


    /// Binds a key to an action on a specific page, returns whether the binding was present
    pub fn bind_local(&mut self, key: i32, action: u32, page: usize) -> bool {
        self.pages[page].borrow_mut().bind(key, action)
    }


    /// Unbinds a key from an action on a specific page
    pub fn unbind_local(&mut self, key: i32, action: u32, page: usize) -> bool {
        self.pages[page].borrow_mut().unbind(key, action)
    }
}
impl Drop for UI {
    fn drop(&mut self) { //allows ncurses to stop when the UI is deallocated, possibly even during panics
        ncurses::endwin();
    }
}



/// Used to communicate text to SerpentWriter
#[derive(Clone)]
pub struct ColorText {
    pub location: (usize, usize), //location to start printing the text on the screen
    pub text: String,
    pub foreground: Color,
    pub background: Color,    
}
impl ColorText {
    pub fn new(text: String, foreground: Color, background: Color, location: (usize, usize)) -> Self {
        ColorText { location, text, foreground, background}
    }
    
    pub fn simple(text: String) -> Self {
        ColorText { location: (0, 0), text: text, foreground: Color::Default, background: Color::Default}
    }
}



/// Allows the user to bind a key to an action
//#[derive(Clone, Debug)]
//pub struct Keybind {
//    pub key: i32,
//    pub action: u32,
//}
//impl Keybind {
//    pub fn new(key: i32, action: u32) -> Self {
//        Keybind { key, action }
//    }    
//}



/// Tells the user what type of event Serpent has received
#[derive(Clone, Debug)]
pub struct SerpentEvent {
    pub input: InputType, //mouse or keyboard
    pub actions: Vec<u8>, //if any actions are called, they will be listed here
}



/// Helper for SerpentEvent, distinguishes between a keypress and a mouse event
#[derive(Clone, Debug)]
pub enum InputType {
    Key(i32),
    Mouse{x: i32, y: i32, event: u32},
}



/// Enum used for color declarations
#[derive(Clone)]
pub enum Color {
    Default,
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
}



/// Used as an element's interface with ncurses
#[derive(Clone)]
pub struct SerpentWriter {
    messages: Vec<ColorText>, //stores the messages from one element
}
impl SerpentWriter {
    pub fn print(&mut self, message: ColorText) {
        self.messages.push(message);
    }
}



/// Base trait for all elements in serpent, allows user to define their own elements
pub trait SerpentElement {
    fn show(&self, output: &SerpentWriter); 
}







