fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} \n255\n", nx);
    for j in 0..(ny -1) {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = (ny - j) as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
