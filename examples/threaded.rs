#![feature(scoped)]
extern crate disque;

use std::thread;
use disque::{ReadOptions,WriteOptions};

fn main() {
    let hosts = vec![
        "redis://127.0.0.1:7711",
        "redis://127.0.0.1:7712",
        "redis://127.0.0.1:7713"
    ];

    let hosts2 = hosts.clone();

    let _addjob = thread::scoped(|| {
        let mut client = disque::Client::new(hosts).unwrap();
        let opt = WriteOptions::new();

        for i in 1..10 {
            let job_id = client.push("queue", &format!("job {}", i), &opt);
            println!("Job added: {}", job_id);

        }
    });

    let _worker = thread::scoped(|| {
        let mut client = disque::Client::new(hosts2).unwrap();

        let opt = ReadOptions::new();

        for i in 1..10 {
            let jobs = client.fetch(&["queue"], &opt, |queue, _job_id, job| {
                println!("Got a job from queue {:?}: {:?}", queue, job);
                true
            });
            println!("{}. Handled {} jobs in a batch", i, jobs);
        }
    });
}
