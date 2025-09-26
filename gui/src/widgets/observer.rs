use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Click,
    // Hover,
    // Focus
}

/// A subscriber (listener) has type of a callable function.
pub type Subscriber = Rc<RefCell<dyn FnMut()>>;

/// Publisher sends events to subscribers (listeners).
#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber){
        self.events.entry(event_type).or_default().push(listener);
    }
    //TODO think about unsubscribe method
    pub fn unsubscribe(& mut self, event_type: Event, listener: Subscriber){
        // match self.events.get_mut(&event_type) {
        //     Some(_event) => {
        //         _event.retain(|&x| !std::ptr::fn_addr_eq(x, listener));
        //     }
        //     None => println!("No Clinet to unsubscribe.")
        // }
        println!("should unsubscribe");
    }

    pub fn notify(&mut self, event_type: Event){
        match self.events.get_mut(&event_type){
            Some(subscriber_vec) => {
                for listener in subscriber_vec.iter_mut() {
                    (listener.borrow_mut())();
                }
            }
            None => println!("Non clients to be notified")
        }        
    }
}
