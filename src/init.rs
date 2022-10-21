use crate::vga;


pub fn main() {

    vga::print!("Text printed to screen.\n");

    vga::colour!(Red, Cyan);
    vga::print!("Hello ");
    vga::colour!(Blue, Green);
    vga::print!("World!");
    vga::colour!();
    vga::print!("\n");

    let mut executor = crate::tasks::Executor::new();
    executor.spawn(crate::tasks::task::Task::new(example_task()));
    executor.spawn(crate::tasks::task::Task::new(crate::tasks::task::keyboard::print_keypresses()));
    executor.run();

    vga::colour!(Pink, Magenta);
    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

}


async fn async_number() -> u32 {
    return 42;
}
async fn example_task() {
    let number = async_number().await;
    vga::print!("async num {}\n", number);
}
