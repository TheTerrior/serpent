pub mod internal;
pub mod error;
pub mod elements;
pub mod constants;

use std::{cell::RefCell, rc::Rc};

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
    //window: ncurses::WINDOW,
    //mouse_events: ncurses::MEVENT,
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


    ///// Gets user input and takes the correct action TODO
    //fn get_input(&self) -> SerpentResult {
    //    SerpentResult::Exit
    //}

    pub fn next(&self) -> SerpentEvent {
        ncurses::refresh(); //refresh the screen
        let event = ncurses::getch();
        if event == 409 { //if received a mouse event
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
    fn print(&mut self, message: ColorText, output: &mut SerpentWriter) {
        self.messages.push(message);
    }
}



/// Base trait for all elements in serpent, allows user to define their own elements
pub trait SerpentElement {
    fn show(&self, output: &SerpentWriter); 
}







