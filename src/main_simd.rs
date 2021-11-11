use std::time::Instant;
use packed_simd::u32x8 as u32s;

fn main() {
    //let start = Instant::now();
    let mut arr: [u32; 8] = [0, 0, 0, 1, 2, 3, 4, 5];
    //let mut arrs = u32s::from_slice_unaligned(&arr);
    //dbg!(arrs);
    for _ in 0..10 {
        for i in 1..8 {
            arr[i] = arr[i] + arr[i-1];
            //println!("hi");
        }
        //arrs = arrs + arrs;
    }
    println!("{:?}", arr);
    //dbg!(arrs);
    //arrs.write_to_slice_unaligned(&mut arr[..]);
    //dbg!(arr);
    //let elapsed = start.elapsed();
    //println!("{} ms", elapsed.as_micros());
}
