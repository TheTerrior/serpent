use std::{error::Error, fmt::Display};

#[derive(Debug)]
enum SerpentError {
    MultipleGuides,     //Multiple guides declared in a page
    MultipleSelectors,  //Multiple selectors declared in a page
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
        write!(f, "Error has occurred: {}", 
            match &self {
                SerpentError::MultipleGuides => "MultipleGuides, only one guide is allowed per window.",
                SerpentError::MultipleSelectors => "MultipleSelectors, only one selector is allowed per window.",
                SerpentError::SplitOutOfBounds => "SplitOutOfBounds, a split was given a value outside of the range [0.0, 1.0]."
            }
        )
    }
}