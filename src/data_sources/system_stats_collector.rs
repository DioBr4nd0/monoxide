use super::traits::MetricsCollector;
use crate::config::Config;
use crate::error::AppError;
use crate::state::data_types::{SystemStatsData, SystemStats};

use sysinfo::{System};
use std::sync::Arc;

pub struct SystemStatsCollector {
    system: System,
}

impl MetricsCollector for SystemStatsCollector {
    type CollectedData = SystemStatsData;

    fn new(config: Arc<Config>) -> Result<Self, AppError> where Self: Sized {
        Ok(SystemStatsCollector { system: System::new() })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {
        self.system.refresh_all();

        let hostname = sysinfo::System::host_name();
        let uptime_secs = sysinfo::System::uptime();
        let load_avg = sysinfo::System::load_average();
        let os_version = sysinfo::System::os_version();
        let kernel_version = sysinfo::System::kernel_version();

        Ok(SystemStatsData::Global(SystemStats { hostname, uptime_secs, load_average: (load_avg.one,load_avg.five ,load_avg.fifteen), os_version, kernel_version, total_users: None }))
    }
}