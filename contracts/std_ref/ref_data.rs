use scale::{Decode, Encode};

#[derive(Encode, Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]

pub struct RefDatum {
    pub rate: u64,
    pub resolve_time: u64,
    pub request_id: u64,
}

impl RefDatum {
    pub fn new(rate: u64, resolve_time: u64, request_id: u64) -> Self {
        Self {
            rate,
            resolve_time,
            request_id,
        }
    }

    pub fn update(&mut self, rate: u64, resolve_time: u64, request_id: u64) {
        if self.resolve_time < resolve_time {
            self.rate = rate;
            self.resolve_time = resolve_time;
            self.request_id = request_id;
        }
    }
}
