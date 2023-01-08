use std::{cell::RefCell, rc::Rc, collections::{HashMap, HashSet}};

use crate::{error::{self, SerpentError}, Color, SerpentWriter, SerpentElement, ElementType};


/// Represents a single page, which can contain a selector and a guide
#[derive(Clone)]
pub struct Page {
    pub partitions: Vec<PartitionInfo>, //allows verification information to be abstracted from a Partition
    pub actions: HashMap<i32, HashSet<u32>>, //binds a phyiscal key to an action
    lazy_focus: Option<usize>, //the default focused lazy partition
    live_focus: Option<usize>, //the live partition we're temporarily focused on
}
impl Page {

    /// Initialize a new instance of Page
    pub fn new() -> Page {
        Page {
            partitions: Vec::new(),
            actions: HashMap::new(),
            lazy_focus: None,
            live_focus: None,
        }
    }


    /// Print this page to the screen
    pub fn show(&mut self, output: &mut SerpentWriter) {

        //iterate through all partitions in this page
        for info in &self.partitions {
            let partition = info.partition.clone();
            let offset = info.offset;
            let size = info.size;

            let partition_ref = partition.clone();
            partition_ref.borrow().show(output); //let the writer print its messages to the immutable writer
            Page::print(output, size, offset);
        }
    }


    /// Print a SerpentWriter's contents to the screen, TODO
    fn print(output: &mut SerpentWriter, size: (usize, usize), offset: (usize, usize)) {

    }


    /// Binds a key to an action on this page
    pub fn bind(&mut self, key: i32, action: u32) -> bool {
        let res = self.actions.get_mut(&key);
        match res {
            None => {
                self.actions.insert(key, HashSet::from([action])) != None
            },
            Some(set) => {
                set.insert(action)
            },
        }
    }


    /// Unbinds a key from an action on this page, returns whether the binding was present
    pub fn unbind(&mut self, key: i32, action: u32) -> bool {
        let res = self.actions.get_mut(&key);
        if let Some(set) = res { //only need to unbind if the key is binded at all
            set.remove(&action)
        } else {
            false
        }
    }


    /// Sets a lazy element as the default focus
    pub fn focus_default(&mut self, index: usize) -> Result<(), SerpentError> {
        if let ElementType::Lazy = self.partitions
                .get(index)
                .ok_or(SerpentError::InvalidPartitionIndex)?
                .partition
                .borrow()
                .element.
                ok_or(SerpentError::NoElementInPartition)?
                .get_type() {
            self.lazy_focus = Some(index);
            return Ok(());
        }
        Err(SerpentError::FocusTypeIncorrect)
    }


    /// Temporarily focus a live partition
    pub fn focus_live(&mut self, index: usize) -> Result<(), SerpentError> {
        if let ElementType::Live = self.partitions
                .get(index)
                .ok_or(SerpentError::InvalidPartitionIndex)?
                .partition
                .borrow()
                .element.
                ok_or(SerpentError::NoElementInPartition)?
                .get_type() {
            self.live_focus = Some(index);
            return Ok(());
        }
        Err(SerpentError::FocusTypeIncorrect)
    }
}



/// Helper struct for the partitions list in Page
#[derive(Clone)]
pub struct PartitionInfo {
    pub partition: Rc<RefCell<Partition>>,
    pub offset: (usize, usize),
    pub size: (usize, usize),
}
impl PartitionInfo {
    pub fn new(partition: Rc<RefCell<Partition>>, offset: (usize, usize), size: (usize, usize)) -> Self {
        PartitionInfo {partition, offset, size}
    }
}



/// One partition of a page's full area
#[derive(Clone)]
pub struct Partition {
    parent: Rc<RefCell<Page>>,  //a link to the partition's parent
    index: usize, //the index of this partition in the page's list of partitions
    pub element: Option<&'static dyn SerpentElement>,   //the element this partition points to, must implement SerpentElement
}
impl Partition {

    /// Create a new partition, handled automatically by a page
    pub fn new(parent: Rc<RefCell<Page>>, index: usize) -> Self {
        Partition { parent: parent, index: index, element: None }
    }


    /// Get the size of this partition
    pub fn get_size(&self) -> (usize, usize) {
        self.parent.borrow().partitions[self.index].size
    }


    /// Split this partition into multiple new partitions, TODO
    pub fn split<const N: usize>(mut self) -> [Self; N] {
        let mut ret: [Partition; N] = unsafe {std::mem::MaybeUninit::uninit().assume_init()}; //initialize empty array

        // for each item in the length
        for i in 0..N {
            ret[i] = Partition::new(self.parent.clone(), 0);
        }
        ret
    }


    /// Call the internal element's show method
    pub fn show(&self, output: &SerpentWriter) {
        if let Some(elem) = self.element {
            elem.show(output);
        }
    }


    /// Binds a key to an action on this page, returns whether the binding was present
    pub fn bind_local(&mut self, key: i32, action: u32, page: usize) -> bool {
        self.parent.borrow_mut().bind(key, action)
    }


    /// Unbinds a key from an action on this page
    pub fn unbind_local(&mut self, key: i32, action: u32, page: usize) -> bool {
        self.parent.borrow_mut().unbind(key, action)
    }
}

