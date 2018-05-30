#![feature(proc_macro)]
 #![feature(extern_prelude)]
extern crate rok;
extern crate rok_derive;
use rok_derive::*;


extern crate glib;
extern crate gtk;

use gtk::{Button, Inhibit, Label, Window, WindowType};
use gtk::{ButtonExt, ContainerExt, GtkWindowExt, LabelExt, WidgetExt};
use rok::*;

use std::marker::{Send, Sync};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

mod gtkframework;
use gtkframework::Gtk;

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
impl rok::Message for Message{}


#[implement_component(Model,Message,Comp)]
pub struct Comp{
    view: gtk::Window,
    label: gtk::Label,
    model: Model,
}
unsafe impl std::marker::Send for Comp {}
unsafe impl std::marker::Sync for Comp {}


impl ComponentBuilder for Comp {
    type Message=Message;
    type Component=Comp;
    type Model=Model;

    fn new(emiter : Sender<Message>) -> Comp {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let sender1 = emiter.clone();
        button.connect_clicked(move |_| {
            let _ = sender1.send(Message::Inc(2));
        });

        let label = Label::new("blaaaaa");

        let bo = gtk::Box::new(gtk::Orientation::Vertical, 2);

        bo.add(&button);
        bo.add(&label);
        window.add(&bo);
        Comp {
            view: window,
            label: label.clone(),
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
       self.view.show_all();
    }


    fn update(&mut self, event: Message) {
        match event {
            Message::Inc(v) => {
                self.model.counter += v;
                println!("{:?}", v);
                self.label.set_text(&format!("{}",self.model.counter));
            }
            Message::Dec(v) => {
                self.model.counter -= v;
            }
            Message::Quit() => {
                //self.view.close();
            }
        };
    }
}





fn main() {
    let mut fw: Gtk = Gtk{};
    FwEventloop::<Message>::init_framework(&mut fw);

    let c = CompWrapper::new();
    c.init();
    fw.eveltloop(c.component.clone(),c.receiver.clone());

    gtk::main();
}
