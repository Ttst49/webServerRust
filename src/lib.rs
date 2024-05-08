use std::thread;


pub struct ThreadPool{
    operators:Vec<Operator>,
}

impl ThreadPool {
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let mut operators = Vec::with_capacity(size);
        for id in 0..size {
            operators.push(Operator::new(id))
        }
        ThreadPool{operators}
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce()+ Send + 'static,{

    }
}

struct Operator{
    id:usize,
    task:thread::JoinHandle<()>
}

impl Operator {
    fn new(id:usize)->Operator{
        let task = thread::spawn(||{});
        Operator{id, task}
    }
}