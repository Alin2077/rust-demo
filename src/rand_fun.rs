use rand::Rng;
use rand::distributions::{Distribution, Uniform, Standard};
use rand_distr::{NormalError, Normal};
use rand::thread_rng;

#[allow(dead_code)]
/// # 生成随机数,不分范围
pub fn rng_no_limit() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random f64: {}", rng.gen::<f64>());
}

#[allow(dead_code)]
/// # 生成范围内随机数
pub fn rng_in_range() {

    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

}

#[allow(dead_code)]
/// ## Uniform模块可以得到均匀分布的值,在相同范围内生成数字是，性能更好
pub fn rng_uniform() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 { break; }
    }
}

#[allow(dead_code)]
/// # 生成给定分布随机数
pub fn rng_distribution() -> Result<(), NormalError> {
    let mut rang = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rang);
    println!("{} is from a N(2,9)", v);
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
/// # 生成自定义类型随机值
struct Point {
    x: i32,
    y: i32,
}

/// ## 下列实现trait是必须的,反之编译器会报错  the trait bound `Standard: Distribution<Point>` is not satisfied
/// ## Standard的sample实现了标准正态分布的随机数,我们需要为其加上结构体的随机数trait才可实现对于结构体的标准随机数
/// ## 详细可咨询 文心一言 AI
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point { x: rand_x, y: rand_y }
    }
}

pub fn rng_standard() {
    let mut rng = thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
}

