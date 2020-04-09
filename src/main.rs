extern crate cgmath;

use cgmath as math;

use std::fs::File;
use std::io;
use std::io::Write;


fn main() -> io::Result<()> {
    let mut file = File::create("output.ppm")?;

    let nx = 200;
    let ny = 100;
    write!(&mut file, "P3\n{} {}\n255\n", nx, ny).unwrap();
    for j in 0..(ny -1) {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = (ny - j) as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}
