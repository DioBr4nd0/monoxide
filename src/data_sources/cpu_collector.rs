use super::traits::MetricsCollector;
use crate::state::data_types::{CpuData, GlobalCpuMetrics};
use crate::error::AppError;
use crate::config::Config; // Using the placeholder Config

use sysinfo::{System};
use crossbeam_channel::Sender;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;

pub struct CpuCollector{
    system: System,
}

impl MetricsCollector for CpuCollector {
    type CollectedData = CpuData;
    fn new(_config:Arc<Config>) -> Result<Self, AppError> {
        Ok(CpuCollector { system: System::new_all() })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {
        self.system.refresh_cpu_usage();

        let total_usage_percent = self.system.global_cpu_usage();
        let core_usages_percent: Vec<f32> = self.system.cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        Ok(CpuData::GlobalCpuMetrics(GlobalCpuMetrics { total_usage_percent, core_usages_percent }))
    }

    fn run_in_thread(
            mut self,
            sender: Sender<Self::CollectedData>,
            shutdown_signal: Arc<AtomicBool>,
            config: Arc<Config>,
        ) -> JoinHandle<()> where Self:Sized {
        std::thread::spawn(move || {
            log::info!("CPU Collector thread started.");
            let mut first_run = true;
            loop{
                if shutdown_signal.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                if first_run {
                    self.system.refresh_cpu_usage();
                    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                    self.system.refresh_cpu_usage();
                    first_run = false;
                } else {
                    self.system.refresh_cpu_usage();
                }

                match self.collect() {
                    Ok(data) => {
                        if sender.send(data).is_err() {
                            log::error!("CPU Collector: Failed to send data, channel closed.");
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("CPU Collector: Error Collecting data: {:?}",e);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(config.refresh_interval_ms));
            }
            log::info!("CPU Collector thread stopped.");
        })
    }
}