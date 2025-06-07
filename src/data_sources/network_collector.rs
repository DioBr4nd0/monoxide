use super::traits::MetricsCollector;
use crate::config::Config;
use crate::error::AppError;
use crate::state::data_types::{NetworkData, GlobalNetworkMetrics};

use sysinfo::{System, Networks};
use std::sync::Arc;


pub struct NetworkCollector {
    networks: Networks,
}

impl MetricsCollector for NetworkCollector {
    type CollectedData = NetworkData;

    fn new(_config: Arc<Config>) -> Result<Self, AppError> {
        Ok(NetworkCollector { networks: Networks::new_with_refreshed_list() })
    }

    fn collect(&mut self) -> Result<Self::CollectedData, AppError> {

        self.networks.refresh(true);
        let metrics: Vec<GlobalNetworkMetrics> = self.networks.list().iter().map(|(name, data)| {
            GlobalNetworkMetrics {
                interface_name: name.clone(),
                received_bytes: data.received(),
                transmitted_bytes: data.transmitted(),
                received_packets: data.packets_received(),
                transmitted_packets: data.packets_transmitted(),
            }
        }).collect();

        Ok(NetworkData::Global(metrics))
    }
}