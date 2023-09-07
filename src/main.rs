// mod  vector_fun;
// use vector_fun::sort_struct as Sort;

// mod rand_fun;
// use rand_fun::rng_alp_range  as Rand;

mod cross_fun;
use cross_fun::find_max as Cross;

fn main() {
    
    // Sort();

    // Rand();

    let arr = &[1, 25, -4, 10];
    let max = Cross(arr);
    println!("max: {:#?}", max);

}
