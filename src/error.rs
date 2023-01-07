use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerpentError {
    MultipleMenus,  //Multiple menus declared in a single page
    SplitOutOfBounds,   //Split boundaries were outside of the range [0.0, 1.0]
    SplitTooSmall,      //Splitting causes one window to have a width or height of 0
    TerminalSizeError, //Error retrieveing the size of the terminal
    InvalidPartitionIndex,
    NoElementInPartition,
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
                SerpentError::SplitOutOfBounds => "SplitOutOfBounds, a split was given a value outside of the range [0.0, 1.0].",
                SerpentError::SplitTooSmall => "SplitTooSmall, splitting caused a partition to have 0 internal area",
                SerpentError::TerminalSizeError => "TerminalSizeError, issue finding the size of the terminal.",
                SerpentError::InvalidPartitionIndex => "InvalidPartitionIndex, partition does not exist at the given index",
                SerpentError::NoElementInPartition => "NoElementInPartition, this partition does not own an element",
            }
        )
    }
}