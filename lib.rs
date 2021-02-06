#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod aura_all;

#[skyline::main(name = "aura_all")]
pub fn main() {
    aura_all::install();
}