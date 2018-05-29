#![recursion_limit="256"]

#![feature(proc_macro)]
extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;

use proc_macro2::Span;
use syn::punctuated::*;
use syn::token::*;
use syn::Visibility::*;
use syn::*;

use std::str::FromStr;


#[macro_use]
extern crate quote;
use quote::TokenStreamExt;

struct ComponentTypes {
    model: syn::Ident,
    message: syn::Ident,
    component: syn::Ident,
}

impl syn::synom::Synom for ComponentTypes {
    named!(parse -> Self, do_parse!(
        model: syn!(syn::Ident) >>
        punct!(,) >>
        message: syn!(syn::Ident) >>
        punct!(,) >>
        component: syn!(syn::Ident) >>
        (ComponentTypes {model, message, component})
    ));
}


#[proc_macro_attribute]
pub fn implement_component(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse(input).unwrap();

    let ComponentTypes {
        model,
        message,
        component,
    } = syn::parse(args).unwrap();

    let sender_type: syn::Type = syn::parse(
        proc_macro::TokenStream::from_str(&format!("std::sync::mpsc::Sender<{}>", message))
            .unwrap(),).unwrap();

    let reciver_type: syn::Type = syn::parse(
        proc_macro::TokenStream::from_str(&format!("std::sync::mpsc::Receiver<{}>", message))
            .unwrap(),).unwrap();

    let model_type: syn::Type =
        syn::parse(proc_macro::TokenStream::from_str(&format!("{}", model)).unwrap()).unwrap();

    let message_type: syn::Type =
        syn::parse(proc_macro::TokenStream::from_str(&format!("{}", message)).unwrap()).unwrap();

    let component_type: syn::Type =
        syn::parse(proc_macro::TokenStream::from_str(&format!("{}", component)).unwrap()).unwrap();

    let component_reciever: syn::Type =
        syn::parse(proc_macro::TokenStream::from_str(&format!("{}Reciever", component)).unwrap()).unwrap();

    let component_wrapper: syn::Type =
         syn::parse(proc_macro::TokenStream::from_str(&format!("{}Wrapper", component)).unwrap()).unwrap();

    let mut expanded = quote! {
        #input
        
        pub struct #component_reciever {
            receiver: std::sync::mpsc::Receiver<#message_type>,
        }
        unsafe impl std::marker::Send for #component_reciever{}
        unsafe impl std::marker::Sync for #component_reciever{}

        impl ComponentRecv for #component_reciever {
            type Message=#message_type;
            fn get_recv(&self) -> &std::sync::mpsc::Receiver<#message_type> {
                &self.receiver
            }
        }



        pub struct #component_wrapper {
            component: std::sync::Arc<std::sync::Mutex<#component>>,
            receiver: std::sync::Arc<std::sync::Mutex<#component_reciever>>,
        }
        impl #component_wrapper {
            pub fn model() -> #model_type {
                #component::model()
            }
            pub fn init(&self) {
                self.component.lock().unwrap().init();
            }
            pub fn update(&self, event: #message_type) {
                self.component.lock().unwrap().update(event);
            }

            pub fn new() -> #component_wrapper{
                  let (sender, receiver) = std::sync::mpsc::channel();

                  #component_wrapper {
                      component: std::sync::Arc::new(std::sync::Mutex::new(#component::new(sender))),
                      receiver: std::sync::Arc::new(std::sync::Mutex::new(#component_reciever{receiver})),
                  }
            }


        }
    };

    expanded.into()
}
