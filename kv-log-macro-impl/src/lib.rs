use ::log::Level;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

extern crate proc_macro;

mod log;

#[proc_macro_hack]
pub fn log(input: TokenStream) -> TokenStream {
    log::log_internal(None, input)
}

#[proc_macro_hack]
pub fn info(input: TokenStream) -> TokenStream {
    log::log_internal(Some(Level::Info), input)
}
