extern crate redis;

use std::collections::HashMap;

mod options;

pub use options::{WriteOptions,ReadOptions};

pub struct Client {
    nodes: HashMap<String, String>,
    client: Option<redis::Client>,
    scout: redis::Client,
    prefix: String
}

#[derive(Debug)]
enum MyType {
    Str(String),
    Int(i64),
    Info(Vec<String>)
}

use MyType::*;

impl redis::FromRedisValue for MyType {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<MyType> {
        match *v {
            redis::Value::Int(val) => Ok(Int(val)),
            redis::Value::Data(ref bytes) => {
                Ok(Str(String::from_utf8(bytes.clone()).unwrap()))
            },
            redis::Value::Bulk(ref items) => {
                let items = items.iter().map(|item| {
                    redis::FromRedisValue::from_redis_value(&item).unwrap()
                }).collect();
                Ok(Info(items))
            },
            _ => panic!("Unknown data type")
        }
    }
}

impl Client {
    pub fn new(hosts: Vec<&'static str>) -> Option<Client> {
        let scout = redis::Client::open(hosts[0]).unwrap();
        let mut c = Client{
            nodes: HashMap::new(),
            client: None,
            scout: scout,
            prefix: "".to_string()
        };

        c.explore(hosts);
        Some(c)
    }


    pub fn push<'b, 'c>(&mut self,
                        queue: &'b str,
                        job: &'c str,
                        options: &WriteOptions) -> String {
        self.pick_client();

        let con = match &self.client {
            &Some(ref c) => c.get_connection().unwrap(),
            _ => panic!("no client :sadface:")
        };

        extend_with_write_options(
            redis::cmd("ADDJOB").arg(queue).arg(job),
            options)
            .query(&con).unwrap()
    }

    pub fn fetch<F>(&mut self,
                        queue: &[&str],
                        options: &ReadOptions,
                        cb: F) -> usize
        where F: Fn(&str, &str, &str) {
        self.pick_client();

        let con = match &self.client {
            &Some(ref c) => c.get_connection().unwrap(),
            _ => panic!("no client :sadface:")
        };

        let jobs : Vec<Vec<String>> = extend_with_read_options(
            redis::cmd("GETJOB").arg("FROM").arg(queue), options)
            .query(&con).unwrap();

        let len = jobs.len();

        for job in jobs {
            cb(&job[0], &job[1], &job[2]);
            self.ackjob(&job[1]);
        }

        len
    }

    fn ackjob(&self, job_id: &str) {
        let con = match &self.client {
            &Some(ref c) => c.get_connection().unwrap(),
            _ => panic!("no client :sadface:")
        };

        redis::cmd("ACKJOB").arg(job_id).execute(&con);
    }

    fn explore(&mut self, hosts: Vec<&'static str>) {
        self.nodes.clear();

        for host in hosts {
            self.scout = match redis::Client::open(host) {
                Ok(c) => c,
                Err(_) => continue
            };

            let con = match self.scout.get_connection() {
                Ok(c) => c,
                Err(_) => continue
            };

            //let (_ok, id, info) : (i32, String, Vec<String>) = redis::cmd("HELLO").query(&con).unwrap();
            let data : Vec<MyType> = redis::cmd("HELLO").query(&con).unwrap();

            self.prefix = match data[1] {
                Str(ref prefix) => From::from(&prefix.clone()[0..8]),
                _ => panic!("data broken")
            };

            for info in &data[2..] {
                let ninfo = match *info {
                    Info(ref d) => d.clone(),
                    _ => panic!("no info")
                };

                let (prefix, host, port, _priority) = (&ninfo[0], &ninfo[1], &ninfo[2], &ninfo[3]);
                self.nodes.insert(From::from(&prefix[0..8]), format!("redis://{}:{}", host, port));
            }

            break;
        }
    }

    fn pick_client(&mut self) -> &Option<redis::Client> {
        if self.client.is_some() {
            return &self.client
        }

        if self.nodes.is_empty() {
            panic!("WAAAAH");
        }

        let next = self.nodes.iter().next().unwrap();
        let c = redis::Client::open(&next.1[..]).unwrap();
        self.client = Some(c);
        &self.client
    }
}


fn extend_with_write_options<'a>(cmd: &'a mut redis::Cmd,
                             options: &WriteOptions) -> &'a redis::Cmd {
    cmd.arg(options.timeout);

    cmd.arg("DELAY").arg(options.delay);
    cmd.arg("MAXLEN").arg(options.delay);

    options.replicate.map(|val| cmd.arg("REPLICATE").arg(val));
    options.retry.map(|val| cmd.arg("RETRY").arg(val));
    options.ttl.map(|val| cmd.arg("TTL").arg(val));

    if options.async {
        cmd.arg("ASYNC");
    }

    cmd
}

fn extend_with_read_options<'a>(cmd: &'a mut redis::Cmd,
                            options: &ReadOptions) -> &'a redis::Cmd {
    cmd.arg("TIMEOUT").arg(options.timeout);
    cmd.arg("COUNT").arg(options.count);
    cmd
}
