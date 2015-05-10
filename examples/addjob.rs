extern crate disque;

use disque::WriteOptions;

fn main() {
    let hosts = vec![
        "redis://127.0.0.1:7711",
        "redis://127.0.0.1:7712",
        "redis://127.0.0.1:7713"
    ];

    let mut opt = WriteOptions::new();
    opt.timeout = 200;
    let mut client = disque::Client::new(hosts).unwrap();

    println!("Job added: {}", client.push("queue", "job", &opt));
}
