use std::option;

use crate::state::data_types::*;

#[derive(Debug, Default)]
pub struct AppState{
    pub cpu: Option<CpuData>,
    pub processes: Option<Vec<ProcessInfo>>,
    pub memory: Option<MemoryData>,
    pub network: Option<NetworkData>,
    pub gpu: Option<GpuData>,
    pub system_stats: Option<SystemStatsData>,
}