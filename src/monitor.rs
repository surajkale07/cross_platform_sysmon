use sysinfo::{System, SystemExt, CpuExt, ProcessExt, NetworksExt, NetworkExt, DiskExt};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

#[derive(Serialize, Clone)]
pub struct Metrics {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    disk_usage: u64,
    disk_total: u64,
    network_received: u64,
    network_transmitted: u64,
}

pub struct Monitor {
    system: Arc<RwLock<System>>,
}

impl Monitor {
    pub fn new() -> Self {
        let system = Arc::new(RwLock::new(System::new_all()));
        Self { system }
    }

    pub async fn gather_metrics(&self) -> Metrics {
        let mut sys = self.system.write().await;
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();
        let disk_usage = sys.disks().iter().map(|d| d.total_space() - d.available_space()).sum();
        let disk_total = sys.disks().iter().map(|d| d.total_space()).sum();
        let network_received = sys.networks().iter().map(|(_, data)| data.received()).sum();
        let network_transmitted = sys.networks().iter().map(|(_, data)| data.transmitted()).sum();

        Metrics {
            cpu_usage,
            memory_used,
            memory_total,
            disk_usage,
            disk_total,
            network_received,
            network_transmitted,
        }
    }

    pub async fn start_monitoring(&self) {
        let alert_threshold = 90.0; // Set threshold for alerts

        loop {
            let metrics = self.gather_metrics().await;

            // Check if any metrics exceed threshold and log an alert
            if metrics.cpu_usage > alert_threshold {
                println!("ALERT: High CPU usage detected: {:.2}%", metrics.cpu_usage);
            }
            if metrics.memory_used as f32 / metrics.memory_total as f32 * 100.0 > alert_threshold {
                println!("ALERT: High memory usage detected.");
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
