use crate::{SerpentElement, SerpentWriter, ElementType};


/// Display basic text on the screen
pub struct Text {

}
impl SerpentElement for Text {
    fn show(&self, output: &SerpentWriter) {
        todo!()
    }
    fn get_type(&self) -> ElementType {
        ElementType::Lazy
    }
}



/// Lets the user select an option from a menu
pub struct MenuSelector {

}
impl SerpentElement for MenuSelector {
    fn show(&self, output: &SerpentWriter) {
        todo!()
    }
    fn get_type(&self) -> ElementType {
        ElementType::Lazy
    }
}

