use std::{error::Error, fmt::Display};

#[derive(Debug)]
enum SerpentError {
    MultipleMenus,  //Multiple menus declared in a single page
    SplitOutOfBounds,   //Split boundaries were outside of the range [0.0, 1.0]
}
impl Error for SerpentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for SerpentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serpent Error: {}", 
            match &self {
                SerpentError::MultipleMenus => "MultipleMenus, only one menu is allowed per window.",
                SerpentError::SplitOutOfBounds => "SplitOutOfBounds, a split was given a value outside of the range [0.0, 1.0]."
            }
        )
    }
}