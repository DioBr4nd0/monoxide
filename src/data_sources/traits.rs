use crate::error::AppError;
use crate::config::Config;
use crossbeam_channel::Sender;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;


pub trait MetricsCollector:Send + 'static {
    type CollectedData: Send + Clone + std::fmt::Debug + 'static;

    fn new(config: Arc<Config>) -> Result<Self, AppError> where Self: Sized;

    fn collect(&mut self) -> Result<Self::CollectedData, AppError>;
    /// Runs the collector in a dedicated thread, periodically collecting
    /// and sending data through the provided sender.
    /// Should respect the shutdown_signal.
    fn run_in_thread(
        mut self,
        sender: Sender<Self::CollectedData>,
        shutdown_signal: Arc<AtomicBool>,
        config: Arc<Config>,
    ) -> JoinHandle<()> where Self:Sized {
        std::thread::spawn(move || {
            log::debug!("Collector thread started for {}", std::any::type_name::<Self>());

            while !shutdown_signal.load(std::sync::atomic::Ordering::Relaxed) {
                match self.collect() {
                    Ok(data) => {
                        if sender.send(data).is_err(){
                            log::error!("Failed to send data from {} collector: channel closed.", std::any::type_name::<Self>());
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Error collecting data in {}: {:?}",std::any::type_name::<Self>(), e);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(config.refresh_interval_ms));
            }
            log::debug!("Collector thread stopped for {}",std::any::type_name::<Self>());
        })
    }
}