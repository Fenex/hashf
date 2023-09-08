use super::*;

pub struct Worker {
    pub manager: Addr<Manager>,
    pub zeroes: u8,
}

impl Actor for Worker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        trace!("[A] Worker started");
        self.manager.do_send(WorkerReady(None));
    }
}

impl Handler<SearchInsideRange> for Worker {
    type Result = ();

    fn handle(
        &mut self,
        SearchInsideRange(range): SearchInsideRange,
        _: &mut Self::Context,
    ) -> Self::Result {
        trace!(
            "[H] START Worker::SearchInsideRange, range is: `{:?}`",
            range
        );

        for i in range.clone() {
            let hash = Sha256::digest(i.to_le_bytes());
            if is_zero_terminated(&hash[..], self.zeroes as usize) {
                self.manager.do_send(HashFound {
                    digit: i,
                    hash: hash.try_into().unwrap(),
                });
            }
        }

        self.manager.do_send(WorkerReady(Some(range.clone())));
        trace!(
            "[H] STOP Worker::SearchInsideRange, range was: `{:?}`",
            range
        );
    }
}
