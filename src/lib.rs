pub mod internal;
use internal::*;
use ncurses as nc;




///
/// SERPENT INTERFACE
/// 

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



///
/// UI INTERFACE
/// 

/// Create a new instance of UI
pub fn new<'a>() -> UI<'a> {
    UI::new()
}

/// Create a new instance of UI from a Page
pub fn from<'a>(page: Page<'a>) -> UI<'a> {
    UI::from_page(page)
}



//
/// PAGE INTERFACE
//

/// Create a new instance of Page
pub fn new_page<'a>(name: &'a str) -> Page<'a> {
    Page::new(name)
}



///
/// UI IMPLEMENTATION
/// 

/// Main controller for Serpent, utilizes ncurses
pub struct UI<'a> {
    root_page: Option<Page<'a>>,
}
impl<'a> UI<'a> {

    /// Create a new instance of UI
    pub fn new() -> UI<'a> {
        UI {
            root_page: None,
        }
    }

    /// Create a new instance of UI from a Page
    pub fn from_page(page: Page<'a>) -> UI<'a> {
        UI {
            root_page: Some(page),
        }
    }

    /// Set the root page for this UI
    pub fn set_page(mut self, page: Page<'a>) -> Self {
        self.root_page = Some(page);
        self
    }

    /// Run the UI and retrieve the result
    pub fn show(&self) -> SerpentResult {
        nc::getch();
        SerpentResult::None
    }

}










// remove this later
pub fn add(left: usize, right: usize) -> usize {
    left + right
}





/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
