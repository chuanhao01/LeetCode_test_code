use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024); // 32MB of stack space

    let handler = builder
        .spawn(|| {
            // stack-intensive operations
        })
        .unwrap();

    handler.join().unwrap();
}
