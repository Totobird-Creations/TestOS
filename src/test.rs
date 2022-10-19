#[cfg(test)]
pub fn runner(tests : &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}