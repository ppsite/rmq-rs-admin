use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RateDetails {
    pub rate: f64,
}

#[derive(Deserialize, Debug)]
pub struct GarbageCollection {
    pub fullsweep_after: u64,
    pub max_heap_size: u64,
    pub min_bin_vheap_size: u64,
    pub min_heap_size: u64,
    pub minor_gcs: u64,
}
