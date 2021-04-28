use std::time::Instant;

fn main() {
    let start = Instant::now();
 
    // Work...
 
    let duration = start.elapsed();

    println!("Time: {:?}", duration);
}
