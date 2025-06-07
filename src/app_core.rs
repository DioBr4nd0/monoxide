use crate::state::app_state::AppState;
use crate::state::data_types::AppStateUpdate;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use crossbeam_channel::Receiver;

pub struct AppCore {
    pub state: Arc<Mutex<AppState>>,
    pub update_receiver: Receiver<AppStateUpdate>,
    pub shutdown_signal: Arc<AtomicBool>,
}

impl AppCore {
    pub fn new(update_receiver: Receiver<AppStateUpdate>, shutdown_signal: Arc<AtomicBool>) -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState::default())),
            update_receiver,
            shutdown_signal,
        }
    }

    pub fn run(&self) {
        while !self.shutdown_signal.load(Ordering::Relaxed) {
            if let Ok(update) = self.update_receiver.recv() {
                let mut state = self.state.lock().unwrap();
                match update {
                    AppStateUpdate::Cpu(data) => state.cpu = Some(data),
                    AppStateUpdate::Processes(data) => state.processes = Some(data),
                    AppStateUpdate::Memory(data) => state.memory = Some(data),
                    AppStateUpdate::Network(data) => state.network = Some(data),
                    AppStateUpdate::Gpu(data) => state.gpu = Some(data),
                    AppStateUpdate::SystemStats(data) => state.system_stats = Some(data),
                }
                // TODO: trigger UI redraw here
            }
        }
    }
}
