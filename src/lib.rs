use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::Mutex;

pub struct ThreadPool{
    operators:Vec<Operator>,
    send:mpsc::Sender<Message>
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
        self.send.send(Message::NewMission(mission)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Send extinction message to all operators.");

        for _ in &self.operators {
            self.send.send(Message::Extinction).unwrap();
        }

        println!("Stop all operators.");

        for operator in &mut self.operators {
            println!("Stopping operator {}", operator.id);

            if let Some(task) = operator.task.take() {
                task.join().unwrap();
            }
        }
    }
}

struct Operator{
    id:usize,
    task:Option<thread::JoinHandle<()>>
}

impl Operator {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Operator {
        let task = thread::spawn(move ||loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewMission(mission)=>{
                    println!("Operator {} got a mission; it execute it",id);
                    mission()
                }
                Message::Extinction=>{
                    println!("Operator {} got a mission; it execute it",id);
                    break;
                }
            }
        });

        Operator { id, task:Some(task) }
    }
}


enum Message{
    NewMission(Mission),
    Extinction
}