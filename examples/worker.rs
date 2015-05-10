extern crate disque;

use disque::ReadOptions;

fn main() {
    let hosts = vec![
        "redis://127.0.0.1:7711",
        "redis://127.0.0.1:7712",
        "redis://127.0.0.1:7713"
    ];

    let mut client = disque::Client::new(hosts).unwrap();

    let mut opt = ReadOptions::new();
    opt.timeout = 200;

    loop {
        let l = client.fetch(&["queue"], &opt, |queue, _job_id, job| {
            println!("Got a job from queue {:?}: {:?}", queue, job);
            true
        });
        println!("Handled {} jobs in a batch", l);
    }
}
