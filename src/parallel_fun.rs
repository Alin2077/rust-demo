use rayon::prelude::*;


/// # 数据并行

/// ## 并行改变数组中元素
pub fn rayon_sample() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|x| *x -= 1);
    println!("{:?}", arr);
}

/// ## 并行测试集合中任意或所有元素是否匹配给定断言
pub fn rayon_any_all() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 )); 
}

/// ## 使用给定断言并行搜索项
pub fn rayon_find() {

    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert_eq!(f3, Some(&8));
}

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

/// ## 对vector并行排序
pub fn vector_sort() {

    let mut vec = vec![String::new(); 10];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric).to_string()).collect()
    });
    //par_sort_unstable()方法是并行的不稳定排序，其中不稳定是指如果两个数相同，他们的相对位置可能会改变
    vec.par_sort_unstable();
    println!("{:#?}",vec);
}

/// ## Map_reduce并行计算
pub fn map_reduce() {

    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];
    //虽然par_iter()是并行，iter()是单线程，但由于涉及到线程同步和调度等开销
    //在数据量小或者操作简单时，iter()的效率可能更高
    let num_over_30 = v.par_iter().map(|x| x.age > 30).count() as f32;
    let sum_over_30 = v.par_iter()
        .map(|x|x.age)
        .filter(|&x|x>30)
        .reduce(||0,|x,y| x+y);

    let alt_sum_30:u32 = v.par_iter()
        .map(|x|x.age)
        .filter(|&x|x>30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    println!("{}",avg_over_30);
    println!("{}",alt_avg_over_30);

    //abs()方法是计算绝对值的方法
    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}

struct Person {
    age: u32,
}
