use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct GlobalStatistics {
    pub avg_consumption: f64,
    pub avg_consumption_last_charge: f64,
    pub num_sessions: i64,
    pub total_distance: i32
}