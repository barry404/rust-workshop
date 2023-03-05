use std::sync::mpsc::{channel, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU32;

pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnMut() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn FnMut() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));


        let handles = (0..num_threads).map(|_| {
            let clone = receiver.clone();
            std::thread::spawn(move || loop {
                let mut work = match clone.lock().unwrap().recv()
                {
                    Ok(work) => work,
                    Err(_) => break
                };
                println!("start");
                work();
                println!("finish");
            })
        }).collect();


        // let mut handles = vec![];
        // for _ in 0..num_threads {
        //     let clone = receiver.clone();
        //     let handle = std::thread::spawn(move || loop {
        //         let work = clone.lock().unwrap().recv().unwrap();
        //         println!("start");
        //         work();
        //         println!("finish");
        //     });
        //     handles.push(handle);
        // }


        Self {
            handles,
            sender,
        }
    }
    pub fn execute<T>(&self, work: T)
        where T: Fnmut + Send + 'static {
        self.sender.send(Box::new(work)).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use super::*;

    #[test]
    fn it_works() {
        let n = AtomicU32::new(0);
        println!("begin");
        let pool = ThreadPool::new(10);
        // let foo= || std::thread::sleep(std::time::Duration::from_secs(1));
        let foo = move || { n.fetch_add(1, Ordering::SeqCst); };
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
