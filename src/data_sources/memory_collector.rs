use super::traits::MetricsCollector;
use crate::config::Config;
use crate::error::AppError;
use crate::state::data_types::{MemoryData, GlobalMemoryMetrics};

use sysinfo::{System};
use std::sync::Arc;

pub struct MemoryCollector {
    system:System,
}

impl MetricsCollector for MemoryCollector {
    type CollectedData = MemoryData;

    fn new(_config: Arc<Config>) -> Result<Self, AppError> {
        Ok(MemoryCollector { system: System::new() })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {
        self.system.refresh_memory();

        let total_memory_kb = self.system.total_memory();
        let available_memory_kb = self.system.available_memory();
        let used_memory_kb = total_memory_kb.saturating_sub(available_memory_kb);
        let total_swap_kb = self.system.total_swap();
        let used_swap_kb = self.system.used_swap();

        Ok(MemoryData::Global(GlobalMemoryMetrics{
            total_memory_kb,
            available_memory_kb,
            used_memory_kb,
            total_swap_kb,
            used_swap_kb,
        }))
    }
}