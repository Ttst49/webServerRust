use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::Mutex;

pub struct ThreadPool{
    operators:Vec<Operator>,
    send:mpsc::Sender<Mission>
}

type Mission = Box<dyn FnOnce()+Send+'static>;


impl ThreadPool {
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut operators = Vec::with_capacity(size);
        for id in 0..size {
            operators.push(Operator::new(id,Arc::clone(&receiver)))
        }
        ThreadPool{operators, send: sender}
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce()+ Send + 'static,{
        let mission = Box::new(f);
        self.send.send(mission).unwrap()
    }
}

struct Operator{
    id:usize,
    task:thread::JoinHandle<()>
}

impl Operator {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Mission>>>) -> Operator {
        let task = thread::spawn(move ||loop {
            let mission = receiver.lock().unwrap().recv().unwrap();
            println!("Operator {} got a mission ; it execute it.", id);

            mission();
        });

        Operator { id, task }
    }
}