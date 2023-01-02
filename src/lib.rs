pub mod internal;
pub mod error;

use std::cell::RefCell;

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
    UI::new()
}



//
/// UI IMPLEMENTATION
//

/// Main controller for Serpent, utilizes ncurses
pub struct UI {
    pages: Vec<RefCell<Page>>,
}
impl UI {

    /// Create a new instance of UI
    pub fn new() -> UI {
        UI {
            pages: Vec::new(),
        }
    }

    /// Create a new page in this UI, returns a mutable reference and the page index
    pub fn new_page(&mut self) -> (usize, RefCell<Partition>) {
        let mut page = Page::new(); //generate a new page
        let page_ref = RefCell::new(page); //place the page into a refcell

        let mut base_partition = Partition {
            parent: page_ref.clone(), //set the parent page of this partition
            size: termsize::get().map(|size| (size.cols as usize, size.rows as usize)).unwrap(), //get the size of the terminal
            offset: (0, 0), //no offset, base partition
            element: None, //no initial element
        };

        page_ref.borrow_mut().partitions = Vec::new(); //push the base partition to the page's list of partitions
        //self.pages.push(RefCell::new(Page::new())); //push the page to the UI's list of pages
        self.pages.push(page_ref.clone());
        //(self.pages.len(), &mut self.pages.last_mut().unwrap().borrow_mut().partitions[0]) //return the mut ref to the partition
        (self.pages.len(), )
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






