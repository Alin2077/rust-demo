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