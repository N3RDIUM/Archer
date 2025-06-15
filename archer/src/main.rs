use archer::compute::manager;

fn main() {
    let result = pollster::block_on(manager::run());
    println!("Hit distance: {result}");
}

