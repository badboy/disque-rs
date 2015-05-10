pub struct WriteOptions {
    /// command timeout in milliseconds
    ///
    /// default: 0 (no timeout)
    pub timeout: u32,

    /// the number of nodes the job should be replicated to
    ///
    /// Default: None (meaning 3, or less if Cluster is smaller)
    pub replicate: Option<u32>,

    /// the number of seconds that should elapse
    /// before the job is queued by any server.
    ///
    /// default: None
    pub delay: Option<u32>,

    /// period after which, if no ACK is received, the job is put again
    /// into the queue for delivery. If RETRY is 0,
    /// the job has at-most-once delivery semantics.
    ///
    /// default: None
    pub retry: Option<u32>,

    /// the max job life in seconds.
    /// After this time, the job is deleted even
    /// if it was not successfully delivered.
    ///
    /// default: None (maximum lifetime chosen by server)
    pub ttl: Option<u32>,

    /// specifies that if there are already count messages queued for
    /// the specified queue name, the message is refused and
    /// an error reported to the client.
    ///
    /// default: None (no limit)
    pub maxlen: Option<u32>,

    /// asks the server to let the command return ASAP and replicate the job
    /// to other nodes in the background.
    ///
    /// The job gets queued ASAP, while normally the job is put into the queue
    /// only when the client gets a positive reply.
    ///
    /// default: false
    pub async: bool
}

impl WriteOptions {
    pub fn new() -> WriteOptions {
        WriteOptions {
            timeout: 0,
            replicate: None,
            delay: None,
            retry: None,
            ttl: None,
            maxlen: None,
            async: false
        }
    }
}

pub struct ReadOptions {
    /// command timeout in milliseconds
    ///
    /// default: 0 (no timeout)
    pub timeout: u32,

    /// number of jobs to return
    ///
    /// default: 1
    pub count: u32,
}

impl ReadOptions {
    pub fn new() -> ReadOptions {
        ReadOptions {
            timeout: 0,
            count: 1
        }
    }
}
