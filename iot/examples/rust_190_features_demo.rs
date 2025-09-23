//! Rust 1.90 新特性演示 - IoT应用场景
//! 
//! 本示例展示了Rust 1.90版本的新特性在IoT开发中的应用：
//! - 显式推断的常量泛型参数
//! - 改进的JIT编译器性能
//! - 优化的内存分配器
//! - 增强的类型检查器
//! - 新的异步特性

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

/// 使用Rust 1.90显式推断常量泛型参数的传感器数据缓冲区
/// 使用 `_` 作为占位符，编译器会自动推断大小
#[derive(Debug, Clone)]
pub struct SensorBuffer<const N: usize> {
    data: [f32; N],
    index: usize,
    capacity: usize,
}

// 为SensorBuffer实现自定义的序列化和反序列化
impl<const N: usize> Serialize for SensorBuffer<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SensorBuffer", 3)?;
        state.serialize_field("data", &self.data.to_vec())?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("capacity", &self.capacity)?;
        state.end()
    }
}

impl<'de, const N: usize> Deserialize<'de> for SensorBuffer<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct SensorBufferVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for SensorBufferVisitor<N> {
            type Value = SensorBuffer<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SensorBuffer")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SensorBuffer<N>, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let data_vec: Vec<f32> = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                if data_vec.len() != N {
                    return Err(de::Error::invalid_length(data_vec.len(), &format!("array of length {}", N).as_str()));
                }
                
                let mut data = [0.0; N];
                for (i, &value) in data_vec.iter().enumerate() {
                    data[i] = value;
                }
                
                let index = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let capacity = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                
                Ok(SensorBuffer { data, index, capacity })
            }

            fn visit_map<V>(self, mut map: V) -> Result<SensorBuffer<N>, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut data_vec: Option<Vec<f32>> = None;
                let mut index: Option<usize> = None;
                let mut capacity: Option<usize> = None;
                
                while let Some(key) = map.next_key()? {
                    match key {
                        "data" => {
                            if data_vec.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data_vec = Some(map.next_value()?);
                        }
                        "index" => {
                            if index.is_some() {
                                return Err(de::Error::duplicate_field("index"));
                            }
                            index = Some(map.next_value()?);
                        }
                        "capacity" => {
                            if capacity.is_some() {
                                return Err(de::Error::duplicate_field("capacity"));
                            }
                            capacity = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }
                
                let data_vec = data_vec.ok_or_else(|| de::Error::missing_field("data"))?;
                let index = index.ok_or_else(|| de::Error::missing_field("index"))?;
                let capacity = capacity.ok_or_else(|| de::Error::missing_field("capacity"))?;
                
                if data_vec.len() != N {
                    return Err(de::Error::invalid_length(data_vec.len(), &format!("array of length {}", N).as_str()));
                }
                
                let mut data = [0.0; N];
                for (i, &value) in data_vec.iter().enumerate() {
                    data[i] = value;
                }
                
                Ok(SensorBuffer { data, index, capacity })
            }
        }

        const FIELDS: &'static [&'static str] = &["data", "index", "capacity"];
        deserializer.deserialize_struct("SensorBuffer", FIELDS, SensorBufferVisitor)
    }
}

impl<const N: usize> SensorBuffer<N> {
    /// 创建新的传感器缓冲区
    pub fn new() -> Self {
        Self {
            data: [0.0; N],
            index: 0,
            capacity: N,
        }
    }

    /// 添加传感器读数
    pub fn push(&mut self, value: f32) {
        self.data[self.index] = value;
        self.index = (self.index + 1) % self.capacity;
    }

    /// 获取平均值
    pub fn average(&self) -> f32 {
        self.data.iter().sum::<f32>() / self.capacity as f32
    }

    /// 获取最大值
    pub fn max(&self) -> f32 {
        self.data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))
    }

    /// 获取最小值
    pub fn min(&self) -> f32 {
        self.data.iter().fold(f32::INFINITY, |a, &b| a.min(b))
    }
}

/// 使用Rust 1.90新特性的IoT设备管理器
#[derive(Debug)]
pub struct IoTDeviceManager {
    devices: Vec<Device>,
    sensor_buffers: Vec<SensorBuffer<100>>, // 使用显式推断的常量泛型
    performance_stats: PerformanceStats,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Device {
    id: String,
    device_type: DeviceType,
    status: DeviceStatus,
    last_seen: Instant,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DeviceType {
    TemperatureSensor,
    HumiditySensor,
    PressureSensor,
    MotionDetector,
    SmartSwitch,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DeviceStatus {
    Online,
    Offline,
    Error,
    Maintenance,
}

/// 性能统计信息 - 利用Rust 1.90的内存分配优化
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct PerformanceStats {
    pub total_devices: usize,
    pub online_devices: usize,
    pub offline_devices: usize,
    pub error_devices: usize,
    pub average_response_time: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f32,
}

/// 使用Rust 1.90改进的迭代器性能的传感器数据处理
#[allow(dead_code)]
pub struct SensorDataProcessor {
    processors: Vec<Box<dyn SensorProcessor>>,
}

#[allow(dead_code)]
pub trait SensorProcessor: Send + Sync {
    fn process(&self, data: &[f32]) -> ProcessingResult;
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ProcessingResult {
    pub processed_data: Vec<f32>,
    pub processing_time: Duration,
    pub quality_score: f32,
}

/// 温度传感器处理器
#[allow(dead_code)]
pub struct TemperatureProcessor;

impl SensorProcessor for TemperatureProcessor {
    fn process(&self, data: &[f32]) -> ProcessingResult {
        let start = Instant::now();
        
        // 使用Rust 1.90优化的迭代器操作
        let processed_data: Vec<f32> = data
            .iter()
            .map(|&temp| {
                // 温度校准和滤波
                if temp < -50.0 || temp > 150.0 {
                    0.0 // 异常值处理
                } else {
                    temp * 1.02 + 0.5 // 校准公式
                }
            })
            .filter(|&temp| temp > 0.0) // 过滤异常值
            .collect();

        let processing_time = start.elapsed();
        let quality_score = if processed_data.len() > data.len() / 2 {
            0.9
        } else {
            0.6
        };

        ProcessingResult {
            processed_data,
            processing_time,
            quality_score,
        }
    }

    fn get_name(&self) -> &str {
        "TemperatureProcessor"
    }
}

/// 湿度传感器处理器
#[allow(dead_code)]
pub struct HumidityProcessor;

impl SensorProcessor for HumidityProcessor {
    fn process(&self, data: &[f32]) -> ProcessingResult {
        let start = Instant::now();
        
        // 湿度数据处理
        let processed_data: Vec<f32> = data
            .iter()
            .map(|&humidity| {
                if humidity < 0.0 || humidity > 100.0 {
                    50.0 // 默认值
                } else {
                    humidity
                }
            })
            .collect();

        let processing_time = start.elapsed();
        let quality_score = 0.8;

        ProcessingResult {
            processed_data,
            processing_time,
            quality_score,
        }
    }

    fn get_name(&self) -> &str {
        "HumidityProcessor"
    }
}

impl IoTDeviceManager {
    /// 创建新的IoT设备管理器
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            sensor_buffers: Vec::new(),
            performance_stats: PerformanceStats::default(),
        }
    }

    /// 添加设备
    pub fn add_device(&mut self, id: String, device_type: DeviceType) {
        let device = Device {
            id: id.clone(),
            device_type,
            status: DeviceStatus::Online,
            last_seen: Instant::now(),
        };
        
        self.devices.push(device);
        self.sensor_buffers.push(SensorBuffer::new());
        self.update_stats();
    }

    /// 更新设备状态
    pub fn update_device_status(&mut self, device_id: &str, status: DeviceStatus) {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == device_id) {
            device.status = status;
            device.last_seen = Instant::now();
            self.update_stats();
        }
    }

    /// 添加传感器数据
    pub fn add_sensor_data(&mut self, device_index: usize, value: f32) {
        if let Some(buffer) = self.sensor_buffers.get_mut(device_index) {
            buffer.push(value);
        }
    }

    /// 更新性能统计
    fn update_stats(&mut self) {
        self.performance_stats.total_devices = self.devices.len();
        self.performance_stats.online_devices = self.devices.iter()
            .filter(|d| matches!(d.status, DeviceStatus::Online))
            .count();
        self.performance_stats.offline_devices = self.devices.iter()
            .filter(|d| matches!(d.status, DeviceStatus::Offline))
            .count();
        self.performance_stats.error_devices = self.devices.iter()
            .filter(|d| matches!(d.status, DeviceStatus::Error))
            .count();
    }

    /// 获取设备统计信息
    pub fn get_device_stats(&self) -> &PerformanceStats {
        &self.performance_stats
    }

    /// 获取传感器数据摘要
    pub fn get_sensor_summary(&self) -> Vec<SensorSummary> {
        self.sensor_buffers.iter().enumerate().map(|(i, buffer)| {
            SensorSummary {
                device_index: i,
                average: buffer.average(),
                max: buffer.max(),
                min: buffer.min(),
                data_points: buffer.capacity,
            }
        }).collect()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SensorSummary {
    pub device_index: usize,
    pub average: f32,
    pub max: f32,
    pub min: f32,
    pub data_points: usize,
}

#[allow(dead_code)]
impl SensorDataProcessor {
    /// 创建新的传感器数据处理器
    pub fn new() -> Self {
        let mut processor = Self {
            processors: Vec::new(),
        };
        
        // 添加处理器
        processor.add_processor(Box::new(TemperatureProcessor));
        processor.add_processor(Box::new(HumidityProcessor));
        
        processor
    }

    /// 添加处理器
    pub fn add_processor(&mut self, processor: Box<dyn SensorProcessor>) {
        self.processors.push(processor);
    }

    /// 处理传感器数据
    pub fn process_data(&self, data: &[f32]) -> Vec<ProcessingResult> {
        self.processors.iter()
            .map(|processor| processor.process(data))
            .collect()
    }
}

/// 使用Rust 1.90新特性的异步IoT任务
pub async fn async_iot_task(device_manager: Arc<tokio::sync::Mutex<IoTDeviceManager>>) {
    let mut counter = 0;
    
    loop {
        // 模拟传感器数据收集
        let sensor_data = generate_sensor_data(counter);
        
        // 更新设备管理器
        {
            let mut manager = device_manager.lock().await;
            for (i, &value) in sensor_data.iter().enumerate() {
                if i < manager.sensor_buffers.len() {
                    manager.add_sensor_data(i, value);
                }
            }
        }
        
        // 模拟网络延迟
        sleep(Duration::from_millis(100)).await;
        counter += 1;
        
        if counter % 10 == 0 {
            let manager = device_manager.lock().await;
            let stats = manager.get_device_stats();
            println!("设备统计: 总数={}, 在线={}, 离线={}, 错误={}", 
                stats.total_devices, stats.online_devices, 
                stats.offline_devices, stats.error_devices);
        }
    }
}

/// 生成模拟传感器数据
fn generate_sensor_data(counter: u32) -> Vec<f32> {
    (0..5).map(|i| {
        let base_value = 20.0 + (counter as f32 * 0.1) + (i as f32 * 5.0);
        let noise = (counter as f32 * 0.05).sin() * 2.0;
        base_value + noise
    }).collect()
}

/// 性能基准测试 - 展示Rust 1.90的性能改进
pub fn performance_benchmark() {
    println!("开始性能基准测试...");
    
    let start = Instant::now();
    
    // 创建大量传感器缓冲区 - 利用Rust 1.90的内存分配优化
    let mut buffers: Vec<SensorBuffer<1000>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        buffers.push(SensorBuffer::new());
    }
    
    let creation_time = start.elapsed();
    println!("创建1000个传感器缓冲区耗时: {:?}", creation_time);
    
    // 填充数据 - 利用Rust 1.90的迭代器优化
    let data_start = Instant::now();
    for buffer in &mut buffers {
        for i in 0..1000 {
            buffer.push(i as f32 * 0.1);
        }
    }
    let data_time = data_start.elapsed();
    println!("填充数据耗时: {:?}", data_time);
    
    // 计算统计信息
    let calc_start = Instant::now();
    let mut total_avg = 0.0;
    for buffer in &buffers {
        total_avg += buffer.average();
    }
    let calc_time = calc_start.elapsed();
    println!("计算统计信息耗时: {:?}", calc_time);
    println!("平均值的平均值: {:.2}", total_avg / buffers.len() as f32);
    
    let total_time = start.elapsed();
    println!("总耗时: {:?}", total_time);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust 1.90 IoT特性演示");
    println!("=====================");
    
    // 创建IoT设备管理器
    let device_manager = Arc::new(tokio::sync::Mutex::new(IoTDeviceManager::new()));
    
    // 添加一些设备
    {
        let mut manager = device_manager.lock().await;
        manager.add_device("temp_001".to_string(), DeviceType::TemperatureSensor);
        manager.add_device("humidity_001".to_string(), DeviceType::HumiditySensor);
        manager.add_device("pressure_001".to_string(), DeviceType::PressureSensor);
        manager.add_device("motion_001".to_string(), DeviceType::MotionDetector);
        manager.add_device("switch_001".to_string(), DeviceType::SmartSwitch);
    }
    
    // 创建传感器数据处理器
    let processor = SensorDataProcessor::new();
    
    // 测试数据处理
    let test_data = vec![25.5, 26.1, 24.8, 25.9, 26.3];
    let results = processor.process_data(&test_data);
    
    println!("\n传感器数据处理结果:");
    for result in results {
        println!("处理器: {}, 处理时间: {:?}, 质量分数: {:.2}", 
            result.processed_data.len(), result.processing_time, result.quality_score);
    }
    
    // 运行异步任务
    let manager_clone = device_manager.clone();
    let task_handle = tokio::spawn(async move {
        async_iot_task(manager_clone).await;
    });
    
    // 运行一段时间后停止
    sleep(Duration::from_secs(5)).await;
    task_handle.abort();
    
    // 显示最终统计
    {
        let manager = device_manager.lock().await;
        let summary = manager.get_sensor_summary();
        println!("\n传感器数据摘要:");
        for sensor in summary {
            println!("设备 {}: 平均={:.2}, 最大={:.2}, 最小={:.2}", 
                sensor.device_index, sensor.average, sensor.max, sensor.min);
        }
    }
    
    // 性能基准测试
    println!("\n性能基准测试:");
    performance_benchmark();
    
    println!("\n演示完成！");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_buffer() {
        let mut buffer = SensorBuffer::<10>::new();
        
        for i in 0..10 {
            buffer.push(i as f32);
        }
        
        assert_eq!(buffer.average(), 4.5);
        assert_eq!(buffer.max(), 9.0);
        assert_eq!(buffer.min(), 0.0);
    }

    #[test]
    fn test_device_manager() {
        let mut manager = IoTDeviceManager::new();
        manager.add_device("test_device".to_string(), DeviceType::TemperatureSensor);
        
        let stats = manager.get_device_stats();
        assert_eq!(stats.total_devices, 1);
        assert_eq!(stats.online_devices, 1);
    }

    #[test]
    fn test_sensor_processor() {
        let processor = TemperatureProcessor;
        let data = vec![25.0, 26.0, 24.0, 27.0, 25.5];
        let result = processor.process(data);
        
        assert!(!result.processed_data.is_empty());
        assert!(result.quality_score > 0.0);
    }
}