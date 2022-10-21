use crate::vga;


pub fn main() {

    vga::print!("Text printed to screen.\n");

    vga::colour!(Red, Cyan);
    vga::print!("Hello ");
    vga::colour!(Blue, Green);
    vga::print!("World!");
    vga::colour!();
    vga::print!("\n");

    vga::colour!(Pink, Magenta);
    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

}
