use super::traits::MetricsCollector;
use crate::config::Config;
use crate::error::AppError;
use crate::state::data_types::{GpuData, GpuMetrics};

use nvml_wrapper::Nvml;
use std::sync::Arc;

pub struct GpuCollector{
    nvml: Nvml,
}

impl MetricsCollector for GpuCollector {
    type CollectedData = GpuData;

    fn new(_config: Arc<Config>) -> Result<Self, AppError> {
        let nvml = Nvml::init().map_err(|e| AppError::CollectionError(format!("Nvml init failed: {:?}",e)))?;
        Ok(GpuCollector { nvml })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {
        let device_count = self.nvml.device_count().map_err(|e| AppError::CollectionError(format!("Device Count Error: {:?}",e)))?;

        let mut gpus = Vec::new();

        for i in 0..device_count{
            let device = self.nvml.device_by_index(i).map_err(|e|AppError::CollectionError(format!("Device Error: {:?}",e)))?;
            let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
            let memory = device.memory_info().ok();
            let utilization = device.utilization_rates().ok();
            let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).ok();

            let fan_speed = device.fan_speed(0).ok();

            gpus.push(GpuMetrics{
                name,
                memory_total_mb: memory.as_ref().map(|m| m.total / 1024 / 1024).unwrap_or(0),
                memory_used_mb: memory.map(|m| m.used / 1024 / 1024).unwrap_or(0),
                utilization_percent: utilization.map(|u| u.gpu).unwrap_or(0),
                temperature_celsius: temperature.unwrap_or(0),
                fan_speed_percent: fan_speed.unwrap_or(0),
            });
        }
        Ok(GpuData::Nvidia(gpus))
    }
}
