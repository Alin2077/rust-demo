
//整数Vector排序
#[allow(dead_code)]
pub fn sort_int() {
    let mut vec = vec![1,5,10,2,15];

    vec.sort();

    println!("{:#?}",vec);
}

//浮点数Vector排序
#[allow(dead_code)]
pub fn sort_float() {
    let mut vec = vec![ 1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a,b| a.partial_cmp(b).unwrap());

    println!("{:#?}",vec);
}

//结构体排序
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

#[allow(dead_code)]
pub fn sort_struct() {

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    //根据自然排序对people排序
    people.sort();
    println!("{:?}",people);

    //自定义根据age排序
    people.sort_by(|a,b| a.age.cmp(&b.age));
    println!("{:?}",people);

}
