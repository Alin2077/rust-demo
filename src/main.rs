// mod  vector_fun;
// use vector_fun::sort_struct as Sort;

// mod rand_fun;
// use rand_fun::rng_alp_range  as Rand;

// mod cross_fun;
// use cross_fun::image_thread as Cross;

// mod parallel_fun;
// use parallel_fun::map_reduce as Rayon;

mod password_fun;
use password_fun::get_sha256 as Pas;


fn main() {
    
    // Sort();

    // Rand();

    // let arr = &[1, 25, -4, 10];
    // let max = Cross(arr);
    // println!("max: {:#?}", max);

    // Cross();

    // Rayon();

    Pas();

}
