pub mod internal;
pub mod error;
pub mod elements;

use std::{cell::RefCell, rc::Rc};

use internal::*;
use error::SerpentError;
use ncurses as nc;
use termsize;


//
/// SERPENT INTERFACE
//

/// Start ncurses
pub fn start() {
    nc::initscr();
}

/// Stop ncurses
pub fn stop() {
    nc::endwin();
}

/// Refresh ncurses
pub fn refresh() {
    nc::refresh();
}

/// Will relaunch ncurses, whether it's currently running or not
pub fn restart() {
    nc::endwin();
    nc::refresh();
    nc::initscr();
}

/// Create a new instance of UI
pub fn new() -> UI {
    nc::initscr(); //start ncurses
    UI::new()
}



//
/// UI IMPLEMENTATION
//

/// Main controller for Serpent, utilizes ncurses
pub struct UI {
    pages: Vec<Rc<RefCell<Page>>>,
}
impl UI {

    /// Create a new instance of UI
    fn new() -> UI {
        UI {
            pages: Vec::new(),
        }
    }

    /// Create a new page in this UI, returns a page index, the partition's size, and the page's base partition
    pub fn new_page(&mut self) -> Result<(usize, Rc<RefCell<Partition>>), SerpentError> {
        let page_ref = Rc::new(RefCell::new(Page::new())); //place a new page into an rc, set its index
        let t_size = termsize::get() //get the size of the terminal
            .map(|size| (size.cols as usize, size.rows as usize))
            .ok_or(SerpentError::TerminalSizeError)?;

        // create the base partition for this new page
        let base_partition = Partition::new(page_ref.clone(), t_size);
        let base_partition_ref = Rc::new(RefCell::new(base_partition)); //place the base partition into an rc

        // push the base partition to the page's list of partitions
        page_ref.borrow_mut().partitions = vec![(base_partition_ref.clone(), (0, 0), t_size)];

        self.pages.push(page_ref); //push the page to the UI's list of pages
        Ok((self.pages.len()-1, base_partition_ref)) //return an rc to the partition
    }


    /// Gets user input and takes the correct action TODO
    fn get_input(&self) -> SerpentResult {
        SerpentResult::Exit
    }
}
impl Drop for UI {
    fn drop(&mut self) { //allows ncurses to stop when the UI is deallocated, possibly even during panics
        nc::endwin();
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







