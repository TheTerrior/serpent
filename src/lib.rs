pub mod internal;
pub mod error;

use std::{cell::RefCell, rc::Rc};

use internal::*;
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



//
/// UI INTERFACE
//

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

    /// Create a new page in this UI, returns a page index and the page's base partition
    pub fn new_page(&mut self) -> (usize, Rc<RefCell<Partition>>) {
        let page_ref = Rc::new(RefCell::new(Page::new())); //place a new page into an rc

        // create the base partition for this new page
        let base_partition = Partition {
            parent: page_ref.clone(), //set the parent page of this partition
            size: termsize::get().map(|size| (size.cols as usize, size.rows as usize)).unwrap(), //get the size of the terminal
            offset: (0, 0), //no offset, base partition
            element: None, //no initial element
        };
        let base_partition_ref = Rc::new(RefCell::new(base_partition)); //place the base partition into an rc

        page_ref.borrow_mut().partitions = vec![base_partition_ref.clone()]; //push the base partition to the page's list of partitions
        self.pages.push(page_ref); //push the page to the UI's list of pages
        (self.pages.len(), base_partition_ref) //return an rc to the partition
    }


    /// Run the UI and retrieve the result
    pub fn show(&self) -> SerpentResult {
        loop {
            self.render();
            let res = self.get_input();

            // if our result is None, then we can safely iterate again
            if let SerpentResult::None = res {
                continue;
            };
            return res;
        }
    }

    /// Renders the current frame
    fn render(&self) {
        let width = termsize::get().unwrap().cols;
        let height = termsize::get().unwrap().rows;
    }

    /// Gets user input and takes the correct action
    fn get_input(&self) -> SerpentResult {
        SerpentResult::Exit
    }
}
impl Drop for UI {
    fn drop(&mut self) { //allows ncurses to stop when the UI is deallocated, possible even during panics
        nc::endwin();
    }
}


/// Enum used for color declarations
#[derive(Clone)]
pub enum Color {
    Inherit,
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Magent,
    Yellow,
}






