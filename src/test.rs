#[cfg(test)]
use crate::vga;

#[cfg(test)]
pub fn runner(tests : &[&dyn Fn()]) {
    vga::colour!(Cyan, Black);
    vga::print!("Running {} tests.", tests.len());
    vga::colour!();
    vga::print!("\n");
    for i in 0..tests.len() {
        let test = tests[i];
        test();
        vga::colour!(LightGreen, Black);
        vga::print!("[OK]");
        vga::colour!();
        vga::print!("\n");
    }
}



#[test_case]
fn trivial() {
    vga::print!("trivial ... ");
    assert_eq!(1, 1);
}

#[test_case]
fn exception() {
    vga::print!("exception ... ");
    // CPU Interrupt
    //x86_64::instructions::interrupts::int3();
    // Double Fault
    /*unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };*/
    // TODO : Stack overflow test
}
