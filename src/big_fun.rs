//针对大文件进行读取数据并计算SHA256,测试
use std::io;  
use std::io::BufReader;  
use std::sync::Arc;  
use std::sync::Mutex;  
use sha2::{SHA256, Digest};  
use std::thread;  
  
fn sha256_digest<R: io::Read + Send + 'static>(mut reader: R, buffer: Arc<Mutex<Vec<u8>>>) -> io::Result<String> {  
    let mut context = SHA256::new();  
    let mut buffer = [0; 8192];  
  
    let mut buf_reader = BufReader::new(&mut reader);  
  
    loop {  
        let count = buf_reader.read(&mut buffer)?;  
        if count == 0 {  
            break;  
        }  
        buffer.lock().unwrap().extend(&buffer[..count]); // 将数据追加到共享缓冲区中  
    }  
  
    // 在所有线程完成后，计算摘要  
    let data = buffer.lock().unwrap();  
    context.input(data);  
    let result = context.result();  
    Ok(format!("{:x}", result))  
}  
  
fn main() {  
    let buffer = Arc::new(Mutex::new(Vec::new())); // 共享缓冲区  
    let file = "large_file.txt"; // 替换为你的文件路径  
    let file = std::fs::File::open(file).unwrap();  
    let file = std::io::BufReader::new(file);  
    let file_size = file.get_ref().metadata().unwrap().len();  
    let chunk_size = 1024 * 1024; // 每个线程处理 1MB 的数据  
    let num_threads = (file_size + chunk_size - 1) / chunk_size; // 计算需要的线程数量  
    let mut handles = Vec::new();  
  
    for i in 0..num_threads {  
        let start = i * chunk_size;  
        let end = std::cmp::min(start + chunk_size, file_size);  
        let reader = file.by_ref().take(end - start); // 创建子读取器  
        let buffer = Arc::clone(&buffer); // 克隆共享缓冲区以便在每个线程中使用  
        handles.push(thread::spawn(move || {  
            sha256_digest(reader, buffer).unwrap(); // 计算摘要并忽略错误  
        }));  
    }  
  
    // 等待所有线程完成  
    for handle in handles {  
        handle.join().unwrap();  
    }  
}