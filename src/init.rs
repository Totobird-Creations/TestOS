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

    // Working on `https://os.phil-opp.com/double-fault-exceptions/#a-stack-overflow-test`

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

}
