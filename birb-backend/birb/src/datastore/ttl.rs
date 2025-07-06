use tokio::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct TTLData {
    cap: Duration,
    addition: Duration,
    base: Duration,
}

impl TTLData {
    pub fn new(cap: Duration, addition: Duration, base: Duration) -> Self {
        Self {
            cap,
            addition,
            base,
        }
    }

    pub fn ttl(&self) -> TTL {
        TTL::new(*self)
    }
}

#[derive(Clone, Copy)]
pub struct TTL {
    data: TTLData,
    expiration: Instant,
}

impl TTL {
    fn new(data: TTLData) -> Self {
        let expiration = Instant::now() + data.base;

        Self { data, expiration }
    }

    pub fn read(&mut self) {
        let extended = self.expiration + self.data.addition;
        let cap = Instant::now() + self.data.cap;

        self.expiration = cap.min(extended);
    }

    pub fn expired(&self) -> bool {
        self.expiration <= Instant::now()
    }
}
