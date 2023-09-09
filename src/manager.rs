use super::*;

/// chain-builder for Manager actor.
#[derive(Debug, Clone, Default)]
pub struct ManagerBuilder {
    workers_count: Option<usize>,
    records: Option<usize>,
    zeroes: Option<u8>,
}

impl ManagerBuilder {
    pub fn build(self) -> Manager {
        Manager {
            workers_count: self.workers_count.unwrap(),
            records: self.records.unwrap(),
            zeroes: self.zeroes.unwrap(),
            workers: None,
            hashes_found: vec![],
            blocks_calc: vec![],
        }
    }

    // Sets the number of threads
    pub fn with_workers(mut self, workers_count: usize) -> Self {
        self.workers_count = Some(workers_count);
        self
    }

    /// Sets the number of required hashes to be found
    pub fn with_records(mut self, records: usize) -> Self {
        self.records = Some(records);
        self
    }

    /// Sets the number of tailing zeroes of required hashes to be found
    pub fn with_zeroes(mut self, zeroes: u8) -> Self {
        self.zeroes = Some(zeroes);
        self
    }
}

pub struct Manager {
    /// number of threads
    workers_count: usize,
    /// required hashes to be found
    records: usize,
    /// required number of tailing zeroes
    zeroes: u8,
    /// addr to pool of workers
    workers: Option<Addr<Worker>>,
    /// storeges ranges that was calculated and is calculating right now
    blocks_calc: Vec<BlockStatus>,
    /// storages founded hashes
    hashes_found: Vec<HashFound>,
}

impl Manager {
    /// Workers will find required hashes in blocks with this range length
    const BLOCK_SIZE: usize = 100000;
}

impl Actor for Manager {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        trace!("[A] Manager started");
        let addr = ctx.address();
        let zeroes = self.zeroes;
        let workers = SyncArbiter::start(self.workers_count, move || Worker {
            zeroes,
            manager: addr.clone(),
        });
        self.workers = Some(workers);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.hashes_found.sort_by_key(|e| e.digit);
        println!("Results:");
        for found in self.hashes_found.iter().take(self.records) {
            println!("{}", found);
        }
    }
}

impl Handler<WorkerReady> for Manager {
    type Result = ();

    fn handle(&mut self, msg: WorkerReady, ctx: &mut Self::Context) -> Self::Result {
        trace!("[H] START Manager::WorkerReady: {:?}", msg);
        if let WorkerReady(Some(range)) = msg {
            // worker has finished a block, so we need marks this block as finished
            let index = (range.start / Self::BLOCK_SIZE as u128) as usize;
            self.blocks_calc[index] = BlockStatus::Finished;
        }

        if self.hashes_found.len() < self.records {
            // number of founded required hashes is less than we need
            // allocate new block and send to worker one
            let digit_start = (self.blocks_calc.len() * Self::BLOCK_SIZE) as u128;
            self.blocks_calc.push(BlockStatus::Started);
            self.workers.as_ref().unwrap().do_send(SearchInsideRange(
                digit_start..Self::BLOCK_SIZE as u128 + digit_start,
            ));
        } else if !self.blocks_calc.iter().any(|&x| x == BlockStatus::Started) {
            // if:
            //  - 1: we found required number of hashes (or maybe even more)
            //  - 2: all workers has finished its job
            // we stop this actor
            ctx.stop();
        }
        trace!("[H] STOP Manager::WorkerReady");
    }
}

impl Handler<HashFound> for Manager {
    type Result = ();

    fn handle(&mut self, msg: HashFound, _: &mut Self::Context) -> Self::Result {
        info!("[H] START Manager::HashFound: {:?}", &msg);
        self.hashes_found.push(msg);
        trace!("[H] STOP Manager::HashFound");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockStatus {
    Started,
    Finished,
}
