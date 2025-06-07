use super::traits::MetricsCollector;
use crate::config::Config;
use crate::error::AppError;
use crate::state::data_types::{ProcessInfo, ProcessStatus};

use sysinfo::System; // Import necessary traits
use std::sync::Arc;

pub struct ProcessCollector {
    system:System,
}

impl MetricsCollector for ProcessCollector {
    type CollectedData = Vec<ProcessInfo>;

    fn new(_config: Arc<Config>) -> Result<Self, AppError> {
        Ok(ProcessCollector { system: System::new_all() })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {
        self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let processes: Vec<ProcessInfo> = self.system
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_str().unwrap().to_string(),
            cpu_usage_percent: process.cpu_usage(),
            memory_usage_kb: process.memory(),
            status: process.status().into(),
        }).collect();

        Ok(processes)
    }
}