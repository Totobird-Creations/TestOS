use crate::vga;


pub fn main() {

    vga::println!("Text printed to screen.");

    vga::colour!(Red, Cyan);
    vga::println!("Hello ");
    vga::colour!(Blue, Green);
    vga::println!("World!");
    vga::colour!();

    vga::colour!(LightMagenta, Magenta);
    vga::println!("Third line.");
    vga::colour!();

}
