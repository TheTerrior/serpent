use crate::{SerpentElement, SerpentWriter, ElementType, InputType};


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

    fn focused(&mut self) {
        todo!()
    }

    fn unfocused(&mut self) {
        todo!()
    }

    fn action(&mut self, action: u32) -> () {
        todo!()
    }

    fn live_key(&mut self, action: InputType) -> () {
        todo!()
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

    fn focused(&mut self) {
        todo!()
    }

    fn unfocused(&mut self) {
        todo!()
    }

    fn action(&mut self, action: u32) -> () {
        todo!()
    }

    fn live_key(&mut self, action: InputType) -> () {
        todo!()
    }
}



pub struct TextBox {

}
impl SerpentElement for TextBox {
    fn show(&self, output: &SerpentWriter) {
        todo!()
    }

    fn get_type(&self) -> ElementType {
        ElementType::Live
    }

    fn focused(&mut self) {
        todo!()
    }

    fn unfocused(&mut self) {
        todo!()
    }

    fn action(&mut self, action: u32) -> () {
        todo!()
    }

    fn live_key(&mut self, action: InputType) -> () {
        todo!()
    }
}

