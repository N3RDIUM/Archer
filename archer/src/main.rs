use archer::compute_manager;

fn main() {
    let result = pollster::block_on(compute_manager::run());
    println!("Hit distance: {result}");
}

