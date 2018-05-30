use std::marker::{Send, Sync};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use rok::*;


pub struct Gtk {}

impl<M: 'static> FwEventloop<M> for Gtk where M:rok::Message  {

    fn init_framework(&mut self){
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
    }

    fn eveltloop(&mut self, update: Arc<Mutex<Component<Message=M>>>, poll: Arc<Mutex<ComponentRecv<Message=M>>>){

        glib::timeout_add(10, move || {
            let poll = poll.clone();
            let update = update.clone();

            let message = {
                let mesg_comp = poll.lock().unwrap();
                mesg_comp.try_recv()
            };


            if let Ok(value) = message {
                let mut update = update.lock().unwrap();
                update.update(value);
            }
            return glib::source::Continue(true);
        });
    }
}
