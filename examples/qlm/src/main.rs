#![feature(extern_prelude)]
#![feature(proc_macro)]
extern crate rok;
extern crate rok_derive;
use rok::*;
use rok_derive::*;

extern crate qt_widgets;

use qt_widgets::cpp_utils::{CppBox, StaticCast};
use qt_widgets::qt_core::connection::Signal;
use qt_widgets::qt_core::slots::SlotBool;


use std::marker::{Send, Sync, Sized};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

mod qtwidgetsframework;
use qtwidgetsframework::QtWidgetsFramework;

pub struct Model {
    counter: i64,
}

#[derive(Debug)]
pub enum Message {
    Inc(i64),
    Dec(i64),
    Quit(),
}
unsafe impl Send for Message {}
unsafe impl Sync for Message {}
impl rok::Message for Message {}


#[implement_component(Model,Message,Comp)]
pub struct Comp{
    view: cpp_utils::CppBox<qt_widgets::widget::Widget>,
    label: Option<cpp_utils::CppBox<qt_widgets::label::Label>>,
    click: SlotBool<'static>,
    model: Model,
}
unsafe impl std::marker::Send for Comp {}
unsafe impl std::marker::Sync for Comp {}
impl Drop for Comp {
    fn drop(&mut self) {
        // all inner widgets will be delete if self.view is deletetd
        // so we need to explicitly forget about them here
        let x = self.label.take();
        std::mem::forget(x);
    }
}


impl ComponentBuilder for Comp {
    type Message=Message;
    type Component=Comp;
    type Model=Model;

    fn new(emiter : Sender<Message>) -> Comp {
        let window = qt_widgets::widget::Widget::new();
        let mut layout = unsafe { qt_widgets::v_box_layout::VBoxLayout::new_unsafe(window.as_mut_ptr()) };
        let mut btn = qt_widgets::push_button::PushButton::new(&qt_widgets::qt_core::string::String::from_std_str("click"));
        let mut lbl = qt_widgets::label::Label::new(&qt_widgets::qt_core::string::String::from_std_str("blaaaa"));

        let sender1 = emiter.clone();
        let slot1 = SlotBool::new(move |value| {
             println!("click");
             let _ = sender1.send(Message::Inc(2));
        });

        btn.signals().clicked().connect(&slot1);

        unsafe{ layout.add_widget(btn.static_cast_mut() as *mut _)};
        unsafe{ layout.add_widget(lbl.static_cast_mut() as *mut _)};


        // all inner widgets will be delete if self.view is deletetd
        // so we need to explicitly forget about them here
        std::mem::forget(btn);
        std::mem::forget(layout);

        Comp {
            view: window,
            label: Some(lbl),
            click: slot1,
            model: Comp::model(),
        }
    }

    fn model() -> Model {
        Model {
            counter : 0
        }
    }
}

impl Component for Comp{
    type Message = Message;

    fn init(&mut self) {
       self.view.show();
    }


    fn update(&mut self, event: Message) {
        match event {
            Message::Inc(v) => {
                self.model.counter += v;
                println!("{:?}", v);
                if let Some(ref mut label) = self.label {
                    label.set_text(&qt_widgets::qt_core::string::String::from_std_str(format!("{}",self.model.counter)));
                }
            }
            Message::Dec(v) => {
                self.model.counter -= v;
            }
            Message::Quit() => {
            }
        };
    }
}



fn main() {
    qt_widgets::application::Application::create_and_exit(|app| {
        let mut fw = QtWidgetsFramework::new();
        let c = CompWrapper::new();
        c.init();
        fw.eveltloop(c.component.clone(),c.receiver.clone());

        qt_widgets::application::Application::exec()
    });
}
