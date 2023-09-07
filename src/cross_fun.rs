/// # 显示线程
use crossbeam;
use std::thread;
use std::time::Duration;
use crossbeam_channel::{bounded,unbounded};


/// ## 生成短期线程
#[allow(dead_code)] //允许死代码，即允许有方法未被使用
pub fn find_max(arr:&[i32]) -> Option<i32> {

    const THRESHOLD:usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap();
        let max_r = thread_r.join().unwrap();

        Some(max_l.max(max_r))
    }).unwrap().unwrap()

}

/// ## 创建并发的数据管道
#[allow(dead_code)]
pub fn cross_channel_bound() {

    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s|{
        //生产者线程
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}",i);
            }
            //关闭信道——这是退出的必要条件
            // for 循环在工作线程中
            drop(snd1);
        });

        //由2个工作线程处理
        for _ in 0..n_workers {
            //从数据源发送数据到接收器，接收器接收数据
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            //从不同的线程中衍生工人
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                //接收数据，直到信道关闭前
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.",
                        thread::current().id(), msg);
                        sendr.send(msg * 2).unwrap();
                }
            });
        }

        //关闭信道，否则接收器不会关闭
        //退出for循环
        drop(snd2);

        //接收器
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }

    }).unwrap();


}

/// ## 在两个线程间传递数据
#[allow(dead_code)]
pub fn cross_channel_unbound() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }).unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }

}

use lazy_static::lazy_static;
use std::sync::Mutex;

/// ## 保持全局可变状态

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str){
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
    db.push(fruit.to_string());
}

pub fn lazy_mutex() {
    insert("apple");
    insert("orange");
    insert("peach");
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i ,item));
    }
    insert("grape");
}



