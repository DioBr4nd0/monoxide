mod error;
mod config;
mod state {
    pub mod app_state;
    pub mod data_types;
}
mod data_sources {
    pub mod traits;
    pub mod cpu_collector;
    pub mod process_collector;
    pub mod memory_collector;
    pub mod network_collector;
    pub mod gpu_collector;
    pub mod system_stats_collector;
}
mod app_core;

use crate::app_core::AppCore;
use crate::config::Config;
use crate::data_sources::traits::MetricsCollector;
use crate::data_sources::cpu_collector::CpuCollector;
use crate::data_sources::process_collector::ProcessCollector;
use crate::data_sources::memory_collector::MemoryCollector;
use crate::data_sources::network_collector::NetworkCollector;
use crate::data_sources::gpu_collector::GpuCollector;
use crate::data_sources::system_stats_collector::SystemStatsCollector;
use crate::state::data_types::AppStateUpdate;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crossbeam_channel::unbounded;
use std::thread;

fn main() -> Result<(), error::AppError> {
    let app_config = Arc::new(Config::default());
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    let (update_sender, update_receiver) = unbounded::<AppStateUpdate>();

    // CPU collector
    {
        let cpu_sender = update_sender.clone();
        let cpu_shutdown = shutdown_signal.clone();
        let cpu_config = app_config.clone();
        let cpu_collector = CpuCollector::new(cpu_config.clone())?;
        thread::spawn(move || {
            let (cpu_data_sender, cpu_data_receiver) = unbounded();
            CpuCollector::run_in_thread(cpu_collector, cpu_data_sender, cpu_shutdown.clone(), cpu_config);
            while let Ok(cpu_data) = cpu_data_receiver.recv() {
                if cpu_sender.send(AppStateUpdate::Cpu(cpu_data)).is_err() {
                    break;
                }
            }
        });
    }

    // Process collector
    {
        let proc_sender = update_sender.clone();
        let proc_shutdown = shutdown_signal.clone();
        let proc_config = app_config.clone();
        let proc_collector = ProcessCollector::new(proc_config.clone())?;
        thread::spawn(move || {
            let (proc_data_sender, proc_data_receiver) = unbounded();
            ProcessCollector::run_in_thread(proc_collector, proc_data_sender, proc_shutdown.clone(), proc_config);
            while let Ok(proc_data) = proc_data_receiver.recv() {
                if proc_sender.send(AppStateUpdate::Processes(proc_data)).is_err() {
                    break;
                }
            }
        });
    }

    // Memory collector
    {
        let mem_sender = update_sender.clone();
        let mem_shutdown = shutdown_signal.clone();
        let mem_config = app_config.clone();
        let mem_collector = MemoryCollector::new(mem_config.clone())?;
        thread::spawn(move || {
            let (mem_data_sender, mem_data_receiver) = unbounded();
            MemoryCollector::run_in_thread(mem_collector, mem_data_sender, mem_shutdown.clone(), mem_config);
            while let Ok(mem_data) = mem_data_receiver.recv() {
                if mem_sender.send(AppStateUpdate::Memory(mem_data)).is_err() {
                    break;
                }
            }
        });
    }

    // Network collector
    {
        let net_sender = update_sender.clone();
        let net_shutdown = shutdown_signal.clone();
        let net_config = app_config.clone();
        let net_collector = NetworkCollector::new(net_config.clone())?;
        thread::spawn(move || {
            let (net_data_sender, net_data_receiver) = unbounded();
            NetworkCollector::run_in_thread(net_collector, net_data_sender, net_shutdown.clone(), net_config);
            while let Ok(net_data) = net_data_receiver.recv() {
                if net_sender.send(AppStateUpdate::Network(net_data)).is_err() {
                    break;
                }
            }
        });
    }

    // GPU collector
    {
        let gpu_sender = update_sender.clone();
        let gpu_shutdown = shutdown_signal.clone();
        let gpu_config = app_config.clone();
        let gpu_collector = GpuCollector::new(gpu_config.clone())?;
        thread::spawn(move || {
            let (gpu_data_sender, gpu_data_receiver) = unbounded();
            GpuCollector::run_in_thread(gpu_collector, gpu_data_sender, gpu_shutdown.clone(), gpu_config);
            while let Ok(gpu_data) = gpu_data_receiver.recv() {
                if gpu_sender.send(AppStateUpdate::Gpu(gpu_data)).is_err() {
                    break;
                }
            }
        });
    }

    // System Stats collector
    {
        let sys_sender = update_sender.clone();
        let sys_shutdown = shutdown_signal.clone();
        let sys_config = app_config.clone();
        let sys_collector = SystemStatsCollector::new(sys_config.clone())?;
        thread::spawn(move || {
            let (sys_data_sender, sys_data_receiver) = unbounded();
            SystemStatsCollector::run_in_thread(sys_collector, sys_data_sender, sys_shutdown.clone(), sys_config);
            while let Ok(sys_data) = sys_data_receiver.recv() {
                if sys_sender.send(AppStateUpdate::SystemStats(sys_data)).is_err() {
                    break;
                }
            }
        });
    }

    let app_core = AppCore::new(update_receiver, shutdown_signal.clone());
    app_core.run();

    Ok(())
}
