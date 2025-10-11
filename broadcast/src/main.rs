use crate::examples::{mpsc1, mpsc2::mpsc2, nonblock::non_block_rev};

mod examples;

fn main() {
    mpsc1::mpsc1();
    mpsc2();

    non_block_rev();
}
