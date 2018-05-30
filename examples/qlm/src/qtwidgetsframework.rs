use qt_widgets::cpp_utils::{CppBox, StaticCast};
use qt_widgets::qt_core::connection::Signal;
use qt_widgets::qt_core::slots::SlotBool;

use std::marker::{Send, Sync};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use rok::*;


pub struct QtWidgetsFramework<'a> {
    timer: cpp_utils::CppBox<qt_widgets::qt_core::timer::Timer>,
    timeout: qt_widgets::qt_core::slots::SlotNoArgs<'a>,
}
impl<'a> QtWidgetsFramework<'a> {
    pub fn new() -> Self{
            let mut t = qt_widgets::qt_core::timer::Timer::new();
            let s = qt_widgets::qt_core::slots::SlotNoArgs::new(||{});
            QtWidgetsFramework {timer:t, timeout:s}
    }
}

impl<'a,M:'a> FwEventloop<M> for QtWidgetsFramework<'a> where M:Message {

    fn init_framework(&mut self){

    }

    fn eveltloop(&mut self, update: Arc<Mutex<Component<Message=M>>>, poll: Arc<Mutex<ComponentRecv<Message=M>>>){
        self.timer.set_interval(10);
        let s = qt_widgets::qt_core::slots::SlotNoArgs::new(move ||{
            let poll = poll.clone();
            let update = update.clone();

            let message = {
                let mesg_comp = poll.lock().unwrap();
                mesg_comp.try_recv()
            };

            if let Ok(value) = message {
                let mut update = update.lock().unwrap();
                update.update(value);
            };
        });

        self.timeout = s;

        self.timer.signals().timeout().connect(&self.timeout);
        self.timer.start(10);
    }
}
