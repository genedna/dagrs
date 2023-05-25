extern crate dagrs;

use dagrs::{DagEngine, EnvVar, Input, Output, SimpleAction, DefaultTask, init_logger};

struct T1 {}

impl SimpleAction for T1 {
    fn run(&self, _input: Input, mut env: EnvVar) -> Output {
        let hello_dagrs = String::from("Hello Dagrs!");
        env.set("you_need_it", hello_dagrs);
        Output::empty()
    }
}

struct T2 {}

impl SimpleAction for T2 {
    fn run(&self, _input: Input, env: EnvVar) -> Output {
        let val = env.get::<String>("you_need_it").unwrap();
        println!("{}", val);
        Output::empty()
    }
}

fn main() {
    // Use dagrs provided logger
    init_logger(None);

    let t1 = DefaultTask::new(T1{}, "Task 1");
    let mut t2 = DefaultTask::new(T2{}, "Task 2");
    let mut dagrs = DagEngine::new();

    // Set up dependencies
    t2.set_predecessors(&[&t1]);

    dagrs.add_tasks(vec![t1, t2]);
    assert!(dagrs.run().unwrap())
}
