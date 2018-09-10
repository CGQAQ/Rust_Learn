// learn to multithread 

use std::sync::{
    Arc,
    Mutex,
};
use std::sync::mpsc::channel;
use std::thread;

fn main(){
    let mut ptr = Arc::new(Mutex::new("hello world!".chars()));
    let (tx, rx) = channel();
    let (etx, erx) = channel();

    thread::spawn(move || {
        loop {
            if let Ok(foo) = rx.try_recv() {
                if foo == '!' {
                    let a = etx.clone();
                    a.send(1);
                }
            }
        }
    });

    loop {
        let mut ptr = ptr.clone();
        
        let mut tx = tx.clone();
        thread::spawn(move || {
            let mut data = ptr.lock().unwrap();
            if let Some(d) = data.next() {
                println!("{}", d);
                tx.send(d);
            }
        });

        if let Ok(a) = erx.try_recv() {
            if a == 1 {
                break
            }
        }
    }
}
