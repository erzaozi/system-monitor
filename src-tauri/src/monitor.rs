use lazy_static;
use std::{
    collections::HashMap,
    sync::Mutex,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use battery::{ Battery, Manager};
use sysinfo::{Component, Disk, Networks, Process, System};

// 全局共享的 System 实例，使用 Mutex 包装
lazy_static::lazy_static! {
    static ref GLOBAL_SYSTEM: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(serde::Serialize, Clone)]
struct NetworkInfo {
    received_rate: u64,    // 下行速度 (bytes/s)
    transmitted_rate: u64, // 上行速度 (bytes/s)
    timestamp: u64,
}

#[derive(serde::Serialize, Default)]
pub struct SysMonitorData {
    host: HostData,
    disks: Vec<DiskData>,
    sensors: HashMap<String, f32>,
}

#[derive(serde::Serialize, Default)]
pub struct ProcessData {
    name: String,
    memory: f64,
    pid: String,
}

impl ProcessData {
    pub fn new(process: &Process) -> Self {
        Self {
            name: process.name().to_str().unwrap_or("Unknown").to_string(),
            memory: MemoryData::format_memory(process.memory()),
            pid: process.pid().to_string(),
        }
    }
}

#[derive(serde::Serialize, Default)]
pub struct SensorData {
    label: String,
    temperature: Option<f32>,
}

impl SensorData {
    pub fn new(component: &Component) -> Self {
        Self {
            label: component.label().to_string(),
            temperature: component.temperature(),
        }
    }
}

#[derive(serde::Serialize, Default, Clone)]
pub struct CpuData {
    chip_name: String,
    physical_core_count: usize,
    global_usage: f32,
    cores: Vec<CpuCoreData>,
}

impl CpuData {
    pub fn new(system: &System, cores: Vec<CpuCoreData>) -> Self {
        let first_cpu = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or("Unknown".to_string());
        Self {
            chip_name: first_cpu,
            physical_core_count: system.physical_core_count().unwrap_or(0),
            global_usage: system.global_cpu_usage(),
            cores,
        }
    }
}

#[derive(serde::Serialize, Default, Clone)]
pub struct CpuCoreData {
    usage: f32,
    frequency: u64,
}

impl CpuCoreData {
    pub fn new(usage: f32, frequency: u64) -> Self {
        Self { usage, frequency }
    }
}

#[derive(serde::Serialize, Default)]
pub struct BatteryData {
    temperature: String,
    cycle_count: u32,
    state: i32,
    percentage: f32,
    state_of_health: String,
}

impl BatteryData {
    pub fn new(battery: &Battery) -> Self {
        Self {
            temperature: format!(
                "{:.2}℃",
                battery.temperature().unwrap_or_default().value - 273.15
            ),
            cycle_count: battery.cycle_count().unwrap_or(0),
            state: match battery.state() {
                battery::State::Full => 1,
                battery::State::Charging => 2,
                battery::State::Discharging => 3,
                battery::State::Empty => 0,
                _ => -1,
            },
            percentage: battery.state_of_charge().value * 100.,
            state_of_health: format!("{:.2}%", battery.state_of_health().value * 100.),
        }
    }
}

#[derive(serde::Serialize, Default)]
pub struct HostData {}

#[derive(serde::Serialize, Default)]
pub struct MemoryData {
    total_memory: f64,
    total_swap: f64,
    used_memory: f64,
    used_swap: f64,
}

impl MemoryData {
    pub fn new(system: &System) -> Self {
        Self {
            total_memory: MemoryData::format_memory(system.total_memory()),
            total_swap: MemoryData::format_memory(system.total_swap()),
            used_memory: MemoryData::format_memory(system.used_memory()),
            used_swap: MemoryData::format_memory(system.used_swap()),
        }
    }

    fn format_memory(bytes: u64) -> f64 {
        bytes as f64 / (1024. * 1024. * 1024.)
    }
}

#[derive(serde::Serialize, Default)]
pub struct DiskData {
    name: String,
}

impl DiskData {
    pub fn new(disk: &Disk) -> Self {
        Self {
            name: format!("{:?}", disk),
        }
    }
}

// 后台任务：用于推送网络流量信息
pub fn start_network_monitoring(window: tauri::Window) {
    thread::spawn(move || {
        loop {
            let mut networks = Networks::new();
            networks.refresh(true);
            // 每秒刷新一次
            thread::sleep(Duration::from_secs(1));
            networks.refresh(true);

            let mut total_received = 0;
            let mut total_transmitted = 0;

            for (_, data) in networks.iter() {
                total_received += data.received();
                total_transmitted += data.transmitted();
            }

            let received_rate = total_received;
            let transmitted_rate = total_transmitted;

            // 推送结果到前端
            if let Err(e) = window.emit(
                "network_info", // 前端事件名
                NetworkInfo {
                    received_rate,
                    transmitted_rate,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs()) // 成功时获取秒数
                        .unwrap_or(0),
                },
            ) {
                eprintln!("Failed to send network info: {:?}", e);
            }
        }
    });
}

#[tauri::command]
pub fn system_info() -> SysMonitorData {
    use sysinfo::Components;
    let components = Components::new_with_refreshed_list();
    let mut sensors = HashMap::new();
    for component in &components {
        sensors.insert(
            component.label().to_string(),
            component.temperature().unwrap_or(0.0),
        );
    }
    SysMonitorData {
        host: HostData::default(),
        disks: vec![], // You can implement Disk collection logic here
        sensors,
    }
}

#[tauri::command]
pub fn memory_info() -> MemoryData {
    let mut system = GLOBAL_SYSTEM.lock().unwrap();
    system.refresh_memory();
    MemoryData::new(&system)
}

#[tauri::command]
pub fn cpu_info() -> CpuData {
    let mut system = GLOBAL_SYSTEM.lock().unwrap();
    system.refresh_cpu_all();

    let cpus = system.cpus();
    let cpu_cores: Vec<CpuCoreData> = cpus
        .iter()
        .map(|cpu| CpuCoreData::new(cpu.cpu_usage(), cpu.frequency()))
        .collect();
    CpuData::new(&system, cpu_cores)
}

#[tauri::command]
pub fn process_info() -> Vec<ProcessData> {
    let mut system = GLOBAL_SYSTEM.lock().unwrap();
    system.refresh_all();

    let mut processes: Vec<ProcessData> = system
        .processes()
        .iter()
        .map(|(_, process)| ProcessData::new(process))
        .collect();

    processes.sort_by(|a, b| b.memory.partial_cmp(&a.memory).unwrap());
    processes
}

#[tauri::command]
pub fn battery_info() -> BatteryData {
    let manager = Manager::new().unwrap();

    for (_, battery) in manager.batteries().unwrap().enumerate() {
        if let Ok(battery) = battery {
            return BatteryData::new(&battery);
        }
    }

    BatteryData::default()
}

#[tauri::command]
pub fn resize_window(window: tauri::Window, height: f64) {
    let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: window.inner_size().unwrap().width as f64,
        height,
    }));
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn update_tray_title() -> bool {
    // do nothing
    return true;
}
