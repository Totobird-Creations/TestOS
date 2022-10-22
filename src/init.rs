use crate::vga;


pub fn main() {

    vga::print!("Text printed to screen.\n");

    vga::colour!(Red, Cyan);
    vga::print!("Hello ");
    vga::colour!(Blue, Green);
    vga::print!("World!");
    vga::colour!();
    vga::print!("\n");

    vga::colour!(LightMagenta, Magenta);
    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

    let mut executor = crate::tasks::Executor::new();
    executor.spawn(crate::tasks::task::Task::new(example_task()));
    executor.spawn(crate::tasks::task::Task::new(crate::tasks::task::keyboard::print_keypresses()));
    executor.run();

}


#[cfg(test)]
pub fn test() {
    crate::init_test();
}


async fn async_number() -> u32 {
    return 42;
}
async fn example_task() {
    let number = async_number().await;
    vga::print!("async num {}\n", number);
}
