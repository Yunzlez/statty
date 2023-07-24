use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct GlobalStatistics {
    pub avg_consumption: f64,
    pub avg_consumption_last_charge: f64
}