// mod  vector_fun;
// use vector_fun::sort_struct as Sort;

// mod rand_fun;
// use rand_fun::rng_alp_range  as Rand;

// mod cross_fun;
// use cross_fun::image_thread as Cross;

// mod parallel_fun;
// use parallel_fun::map_reduce as Rayon;

// mod password_fun;
// use password_fun::get_sha256 as Pas;

// mod time_fun;
// use time_fun::time_calcul as Time;

// mod url_fun;
// use url_fun::extrate_url as Url;

mod mime_fun;
use mime_fun::get_mime as Mime;

fn main() {
    
    // Sort();

    // Rand();

    // let arr = &[1, 25, -4, 10];
    // let max = Cross(arr);
    // println!("max: {:#?}", max);

    // Cross();

    // Rayon();

    // Pas();

    // Time();

    // Url();

    Mime();

}
