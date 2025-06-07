use serde::{Deserialize, Serialize}; // If you plan to serialize later

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCpuMetrics {
    pub total_usage_percent: f32, // Overall CPU usage
    pub core_usages_percent: Vec<f32>, // Usage per logical core
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerProcessCpuMetrics {
    pub pid: u32,
    pub cpu_usage_percent:f32,
}

#[derive(Debug, Clone)]
pub enum CpuData {
    GlobalCpuMetrics(GlobalCpuMetrics)
}

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    Run, 
    Sleep,
    Idle,
    Zombie,
    Unknown(String),
}

// Convert from sysinfo's ProcessStatus to our own, abstracting the dependency.
impl From<sysinfo::ProcessStatus> for ProcessStatus {
    fn from(status: sysinfo::ProcessStatus) -> Self{
        match status {
            sysinfo::ProcessStatus::Run => ProcessStatus::Run,
            sysinfo::ProcessStatus::Sleep => ProcessStatus::Sleep,
            sysinfo::ProcessStatus::Idle => ProcessStatus::Idle,
            sysinfo::ProcessStatus::Zombie => ProcessStatus::Zombie,

            _ => ProcessStatus::Unknown(status.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage_percent: f32,
    pub memory_usage_kb: u64,
    pub status: ProcessStatus,
}

#[derive(Debug, Clone)]
pub struct GlobalMemoryMetrics {
    pub total_memory_kb: u64,
    pub available_memory_kb: u64,
    pub used_memory_kb: u64,
    pub total_swap_kb: u64,
    pub used_swap_kb: u64,
}

#[derive(Debug, Clone)]
pub enum MemoryData {
    Global(GlobalMemoryMetrics),
}

#[derive(Debug, Clone)]
pub struct GlobalNetworkMetrics {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub received_packets: u64,
    pub transmitted_packets: u64,
}

#[derive(Debug, Clone)]
pub enum NetworkData {
    Global(Vec<GlobalNetworkMetrics>), // List for all interfaces
}

#[derive(Debug, Clone)]
pub struct GpuMetrics {
    pub name:String,
    pub memory_total_mb: u64,
    pub memory_used_mb : u64,
    pub utilization_percent: u32,
    pub temperature_celsius: u32,
    pub fan_speed_percent: u32,
}

#[derive(Debug, Clone)]
pub enum GpuData{
    Nvidia(Vec<GpuMetrics>)
}


#[derive(Debug, Clone)]
pub struct SystemStats {
    pub hostname: Option<String>,
    pub uptime_secs: u64,
    pub load_average: (f64, f64, f64),
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
    pub total_users: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum SystemStatsData{
    Global(SystemStats),
}

#[derive(Debug, Clone)]
pub enum AppStateUpdate {
    Cpu(CpuData),
    Processes(Vec<ProcessInfo>),
    Memory(MemoryData),
    Network(NetworkData),
    Gpu(GpuData),
    SystemStats(SystemStatsData),
}