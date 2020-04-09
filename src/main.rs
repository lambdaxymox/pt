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
            let col = math::vec3((i as f32 / nx as f32, (ny - j) as f32 / ny as f32, 0.2));
            let ir = (255.99 * col[0]) as u32;
            let ig = (255.99 * col[1]) as u32;
            let ib = (255.99 * col[2]) as u32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    Ok(())
}
