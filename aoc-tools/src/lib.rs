use std::{fmt, time::SystemTime};

pub fn run_solution<F, T>(func: F, part: i8)
where
    F: Fn() -> T,
    T: fmt::Debug,
{
    let start = SystemTime::now();
    let result = func();
    let elapsed = start.elapsed().unwrap();

    print!("Part {}: {:?} ", part, result);
    println!("\t(took {}ms)\n", elapsed.as_micros() as f64 / 1000.0);
}
