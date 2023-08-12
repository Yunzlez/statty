use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct GlobalStatistics {
    pub avg_consumption: f64,
    pub avg_consumption_last_charge: f64,
    pub num_sessions: i64,
    pub total_distance: i32,
    pub distance_last_charge: i32,
    pub avg_distance_80_percent: f64,
    pub total_energy: f64,
}