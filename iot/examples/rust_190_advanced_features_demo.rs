//! Rust 1.90 高级特性演示 - IoT应用场景
//! 
//! 本示例展示了Rust 1.90版本的高级特性在IoT开发中的应用：
//! - 高级模式匹配
//! - 改进的错误处理
//! - 异步编程增强
//! - 内存安全优化
//! - 性能优化技巧

use std::sync::Arc;
use std::time::{Duration, /*Instant,*/ SystemTime, UNIX_EPOCH};
//use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 高级IoT设备状态管理 - 使用Rust 1.90的新特性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedIoTDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub capabilities: Vec<Capability>,
    pub metrics: DeviceMetrics,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Sensor(SensorType),
    Actuator(ActuatorType),
    Gateway(GatewayType),
    Edge(EdgeType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Motion,
    Light,
    Sound,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActuatorType {
    Relay,
    Motor,
    Valve,
    Light,
    Speaker,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatewayType {
    WiFi,
    Bluetooth,
    Zigbee,
    LoRaWAN,
    Cellular,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    RaspberryPi,
    JetsonNano,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online { uptime: u64, last_ping: u64 },
    Offline { last_seen: u64, reason: OfflineReason },
    Maintenance { scheduled_end: u64, maintenance_type: String },
    Error { error_code: u32, error_message: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfflineReason {
    NetworkFailure,
    PowerFailure,
    HardwareFailure,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub version: String,
    pub parameters: Vec<Parameter>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub default_value: ParameterValue,
    pub constraints: Option<ParameterConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Integer { min: i64, max: i64 },
    Float { min: f64, max: f64 },
    Boolean,
    String { max_length: usize },
    Enum(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Enum(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub required: bool,
    pub read_only: bool,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    Range { min: f64, max: f64 },
    Pattern(String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_usage: NetworkUsage,
    pub power_consumption: f32,
    pub temperature: f32,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub connection_count: u32,
}

/// 高级IoT设备管理器 - 使用Rust 1.90的新特性
#[derive(Debug)]
pub struct AdvancedIoTDeviceManager {
    devices: Arc<tokio::sync::RwLock<Vec<AdvancedIoTDevice>>>,
    event_handlers: Vec<Box<dyn DeviceEventHandler + Send + Sync>>,
    metrics_collector: MetricsCollector,
    rule_engine: RuleEngine,
}

/// 设备事件处理器trait
#[async_trait::async_trait]
pub trait DeviceEventHandler: Send + Sync + std::fmt::Debug {
    async fn handle_device_event(&self, event: DeviceEvent) -> Result<(), DeviceError>;
    fn get_event_types(&self) -> Vec<DeviceEventType>;
}

/// 设备事件类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceEventType {
    StatusChange,
    MetricUpdate,
    ErrorOccurred,
    CapabilityAdded,
    CapabilityRemoved,
    ParameterChanged,
}

/// 设备事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceEvent {
    pub device_id: String,
    pub event_type: DeviceEventType,
    pub timestamp: u64,
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
    StatusChange { old_status: DeviceStatus, new_status: DeviceStatus },
    MetricUpdate { metrics: DeviceMetrics },
    ErrorOccurred { error: DeviceError },
    CapabilityChange { capability: Capability, action: CapabilityAction },
    ParameterChange { parameter_name: String, old_value: ParameterValue, new_value: ParameterValue },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityAction {
    Added,
    Removed,
    Modified,
}

/// 指标收集器
#[derive(Debug)]
#[allow(dead_code)]
pub struct MetricsCollector {
    metrics: Arc<tokio::sync::RwLock<Vec<MetricEntry>>>,
    collection_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEntry {
    pub device_id: String,
    pub metric_name: String,
    pub value: f64,
    pub timestamp: u64,
    pub tags: std::collections::HashMap<String, String>,
}

/// 规则引擎
#[derive(Debug)]
pub struct RuleEngine {
    rules: Vec<Rule>,
    rule_evaluator: RuleEvaluator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    DeviceStatus { device_id: String, status: DeviceStatus },
    MetricThreshold { device_id: String, metric_name: String, operator: ComparisonOperator, threshold: f64 },
    TimeBased { start_time: u64, end_time: u64 },
    Composite { conditions: Vec<RuleCondition>, operator: LogicalOperator },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    SendNotification { message: String, channels: Vec<NotificationChannel> },
    ExecuteCommand { device_id: String, command: String, parameters: Vec<ParameterValue> },
    ChangeDeviceState { device_id: String, new_status: DeviceStatus },
    LogEvent { level: LogLevel, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email(String),
    SMS(String),
    Webhook(String),
    Slack(String),
    Discord(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// 规则评估器
#[derive(Debug)]
#[allow(dead_code)]
pub struct RuleEvaluator {
    context: EvaluationContext,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EvaluationContext {
    pub current_time: u64,
    pub device_states: std::collections::HashMap<String, DeviceStatus>,
    pub device_metrics: std::collections::HashMap<String, DeviceMetrics>,
}

/// 错误类型定义
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum DeviceError {
    #[error("设备未找到: {device_id}")]
    DeviceNotFound { device_id: String },
    
    #[error("设备状态错误: {device_id}, 状态: {status:?}")]
    InvalidDeviceStatus { device_id: String, status: DeviceStatus },
    
    #[error("参数验证失败: {parameter_name}, 值: {value:?}")]
    ParameterValidationFailed { parameter_name: String, value: ParameterValue },
    
    #[error("网络错误: {message}")]
    NetworkError { message: String },
    
    #[error("规则执行失败: {rule_id}, 原因: {reason}")]
    RuleExecutionFailed { rule_id: String, reason: String },
    
    #[error("指标收集失败: {metric_name}, 原因: {reason}")]
    MetricsCollectionFailed { metric_name: String, reason: String },
    
    #[error("事件处理失败: {event_type:?}, 原因: {reason}")]
    EventHandlingFailed { event_type: DeviceEventType, reason: String },
}

impl AdvancedIoTDeviceManager {
    /// 创建新的高级IoT设备管理器
    pub fn new() -> Self {
        Self {
            devices: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            event_handlers: Vec::new(),
            metrics_collector: MetricsCollector::new(Duration::from_secs(30)),
            rule_engine: RuleEngine::new(),
        }
    }

    /// 添加设备
    pub async fn add_device(&self, device: AdvancedIoTDevice) -> Result<(), DeviceError> {
        let mut devices = self.devices.write().await;
        
        // 检查设备是否已存在
        if devices.iter().any(|d| d.id == device.id) {
            return Err(DeviceError::DeviceNotFound { device_id: device.id });
        }
        
        devices.push(device);
        Ok(())
    }

    /// 更新设备状态 - 使用Rust 1.90的高级模式匹配
    pub async fn update_device_status(&self, device_id: &str, new_status: DeviceStatus) -> Result<(), DeviceError> {
        let mut devices = self.devices.write().await;
        
        if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
            let old_status = device.status.clone();
            device.status = new_status.clone();
            device.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            
            // 使用高级模式匹配处理状态变化
            match (&old_status, &new_status) {
                (DeviceStatus::Online { .. }, DeviceStatus::Offline { .. }) => {
                    self.handle_device_event(DeviceEvent {
                        device_id: device_id.to_string(),
                        event_type: DeviceEventType::StatusChange,
                        timestamp: device.last_updated,
                        data: EventData::StatusChange { old_status, new_status },
                    }).await?;
                }
                (DeviceStatus::Offline { .. }, DeviceStatus::Online { .. }) => {
                    self.handle_device_event(DeviceEvent {
                        device_id: device_id.to_string(),
                        event_type: DeviceEventType::StatusChange,
                        timestamp: device.last_updated,
                        data: EventData::StatusChange { old_status, new_status },
                    }).await?;
                }
                (DeviceStatus::Error { .. }, DeviceStatus::Online { .. }) => {
                    self.handle_device_event(DeviceEvent {
                        device_id: device_id.to_string(),
                        event_type: DeviceEventType::StatusChange,
                        timestamp: device.last_updated,
                        data: EventData::StatusChange { old_status, new_status },
                    }).await?;
                }
                _ => {} // 其他状态变化不需要特殊处理
            }
            
            Ok(())
        } else {
            Err(DeviceError::DeviceNotFound { device_id: device_id.to_string() })
        }
    }

    /// 更新设备指标
    pub async fn update_device_metrics(&self, device_id: &str, metrics: DeviceMetrics) -> Result<(), DeviceError> {
        let mut devices = self.devices.write().await;
        
        if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
            device.metrics = metrics.clone();
            device.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            
            // 收集指标
            self.metrics_collector.collect_metrics(device_id, &metrics).await?;
            
            // 触发事件
            self.handle_device_event(DeviceEvent {
                device_id: device_id.to_string(),
                event_type: DeviceEventType::MetricUpdate,
                timestamp: device.last_updated,
                data: EventData::MetricUpdate { metrics },
            }).await?;
            
            Ok(())
        } else {
            Err(DeviceError::DeviceNotFound { device_id: device_id.to_string() })
        }
    }

    /// 添加事件处理器
    pub fn add_event_handler(&mut self, handler: Box<dyn DeviceEventHandler + Send + Sync>) {
        self.event_handlers.push(handler);
    }

    /// 处理设备事件
    async fn handle_device_event(&self, event: DeviceEvent) -> Result<(), DeviceError> {
        for handler in &self.event_handlers {
            if handler.get_event_types().contains(&event.event_type) {
                if let Err(e) = handler.handle_device_event(event.clone()).await {
                    return Err(DeviceError::EventHandlingFailed {
                        event_type: event.event_type,
                        reason: e.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// 获取设备统计信息
    pub async fn get_device_stats(&self) -> DeviceStats {
        let devices = self.devices.read().await;
        
        let total_devices = devices.len();
        let online_devices = devices.iter().filter(|d| matches!(d.status, DeviceStatus::Online { .. })).count();
        let offline_devices = devices.iter().filter(|d| matches!(d.status, DeviceStatus::Offline { .. })).count();
        let error_devices = devices.iter().filter(|d| matches!(d.status, DeviceStatus::Error { .. })).count();
        let maintenance_devices = devices.iter().filter(|d| matches!(d.status, DeviceStatus::Maintenance { .. })).count();
        
        DeviceStats {
            total_devices,
            online_devices,
            offline_devices,
            error_devices,
            maintenance_devices,
        }
    }

    /// 执行规则引擎
    pub async fn execute_rules(&self) -> Result<(), DeviceError> {
        self.rule_engine.evaluate_rules(&self.devices).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStats {
    pub total_devices: usize,
    pub online_devices: usize,
    pub offline_devices: usize,
    pub error_devices: usize,
    pub maintenance_devices: usize,
}

impl MetricsCollector {
    pub fn new(collection_interval: Duration) -> Self {
        Self {
            metrics: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            collection_interval,
        }
    }

    pub async fn collect_metrics(&self, device_id: &str, metrics: &DeviceMetrics) -> Result<(), DeviceError> {
        let mut metric_entries = self.metrics.write().await;
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        
        // 收集各种指标
        let metrics_to_collect = vec![
            ("cpu_usage", metrics.cpu_usage as f64),
            ("memory_usage", metrics.memory_usage as f64),
            ("power_consumption", metrics.power_consumption as f64),
            ("temperature", metrics.temperature as f64),
            ("uptime", metrics.uptime as f64),
        ];
        
        for (metric_name, value) in metrics_to_collect {
            let mut tags = std::collections::HashMap::new();
            tags.insert("device_id".to_string(), device_id.to_string());
            tags.insert("device_type".to_string(), "iot_device".to_string());
            
            metric_entries.push(MetricEntry {
                device_id: device_id.to_string(),
                metric_name: metric_name.to_string(),
                value,
                timestamp,
                tags,
            });
        }
        
        Ok(())
    }

    pub async fn get_metrics(&self, device_id: Option<&str>) -> Vec<MetricEntry> {
        let metrics = self.metrics.read().await;
        
        if let Some(device_id) = device_id {
            metrics.iter().filter(|m| m.device_id == device_id).cloned().collect()
        } else {
            metrics.clone()
        }
    }
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            rule_evaluator: RuleEvaluator::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
        // 按优先级排序
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub async fn evaluate_rules(&self, devices: &Arc<tokio::sync::RwLock<Vec<AdvancedIoTDevice>>>) -> Result<(), DeviceError> {
        let devices = devices.read().await;
        
        // 构建评估上下文
        let mut context = EvaluationContext {
            current_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            device_states: std::collections::HashMap::new(),
            device_metrics: std::collections::HashMap::new(),
        };
        
        for device in devices.iter() {
            context.device_states.insert(device.id.clone(), device.status.clone());
            context.device_metrics.insert(device.id.clone(), device.metrics.clone());
        }
        
        // 评估规则
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if self.rule_evaluator.evaluate_condition(&rule.condition, &context) {
                if let Err(e) = self.execute_action(&rule.action).await {
                    return Err(DeviceError::RuleExecutionFailed {
                        rule_id: rule.id.clone(),
                        reason: e.to_string(),
                    });
                }
            }
        }
        
        Ok(())
    }

    async fn execute_action(&self, action: &RuleAction) -> Result<(), DeviceError> {
        match action {
            RuleAction::SendNotification { message, channels } => {
                println!("发送通知: {}", message);
                for channel in channels {
                    match channel {
                        NotificationChannel::Email(email) => {
                            println!("发送邮件到: {}", email);
                        }
                        NotificationChannel::SMS(phone) => {
                            println!("发送短信到: {}", phone);
                        }
                        NotificationChannel::Webhook(url) => {
                            println!("发送Webhook到: {}", url);
                        }
                        NotificationChannel::Slack(channel) => {
                            println!("发送Slack消息到: {}", channel);
                        }
                        NotificationChannel::Discord(channel) => {
                            println!("发送Discord消息到: {}", channel);
                        }
                    }
                }
            }
            RuleAction::ExecuteCommand { device_id, command, parameters } => {
                println!("执行命令: 设备={}, 命令={}, 参数={:?}", device_id, command, parameters);
            }
            RuleAction::ChangeDeviceState { device_id, new_status } => {
                println!("改变设备状态: 设备={}, 新状态={:?}", device_id, new_status);
            }
            RuleAction::LogEvent { level, message } => {
                println!("记录日志: 级别={:?}, 消息={}", level, message);
            }
        }
        Ok(())
    }
}

impl RuleEvaluator {
    pub fn new() -> Self {
        Self {
            context: EvaluationContext {
                current_time: 0,
                device_states: std::collections::HashMap::new(),
                device_metrics: std::collections::HashMap::new(),
            },
        }
    }

    pub fn evaluate_condition(&self, condition: &RuleCondition, context: &EvaluationContext) -> bool {
        match condition {
            RuleCondition::DeviceStatus { device_id, status } => {
                if let Some(device_status) = context.device_states.get(device_id) {
                    device_status == status
                } else {
                    false
                }
            }
            RuleCondition::MetricThreshold { device_id, metric_name, operator, threshold } => {
                if let Some(metrics) = context.device_metrics.get(device_id) {
                    let value = match metric_name.as_str() {
                        "cpu_usage" => metrics.cpu_usage as f64,
                        "memory_usage" => metrics.memory_usage as f64,
                        "power_consumption" => metrics.power_consumption as f64,
                        "temperature" => metrics.temperature as f64,
                        "uptime" => metrics.uptime as f64,
                        _ => return false,
                    };
                    
                    match operator {
                        ComparisonOperator::Equal => (value - threshold).abs() < f64::EPSILON,
                        ComparisonOperator::NotEqual => (value - threshold).abs() >= f64::EPSILON,
                        ComparisonOperator::GreaterThan => value > *threshold,
                        ComparisonOperator::GreaterThanOrEqual => value >= *threshold,
                        ComparisonOperator::LessThan => value < *threshold,
                        ComparisonOperator::LessThanOrEqual => value <= *threshold,
                    }
                } else {
                    false
                }
            }
            RuleCondition::TimeBased { start_time, end_time } => {
                context.current_time >= *start_time && context.current_time <= *end_time
            }
            RuleCondition::Composite { conditions, operator } => {
                match operator {
                    LogicalOperator::And => conditions.iter().all(|c| self.evaluate_condition(c, context)),
                    LogicalOperator::Or => conditions.iter().any(|c| self.evaluate_condition(c, context)),
                    LogicalOperator::Not => !conditions.iter().any(|c| self.evaluate_condition(c, context)),
                }
            }
        }
    }
}

/// 示例事件处理器
#[derive(Debug)]
pub struct LoggingEventHandler;

#[async_trait::async_trait]
impl DeviceEventHandler for LoggingEventHandler {
    async fn handle_device_event(&self, event: DeviceEvent) -> Result<(), DeviceError> {
        println!("处理设备事件: 设备={}, 类型={:?}, 时间={}", 
            event.device_id, event.event_type, event.timestamp);
        Ok(())
    }

    fn get_event_types(&self) -> Vec<DeviceEventType> {
        vec![
            DeviceEventType::StatusChange,
            DeviceEventType::MetricUpdate,
            DeviceEventType::ErrorOccurred,
        ]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust 1.90 高级特性演示 - IoT应用场景");
    println!("=====================================");
    
    // 创建高级IoT设备管理器
    let mut device_manager = AdvancedIoTDeviceManager::new();
    
    // 添加事件处理器
    device_manager.add_event_handler(Box::new(LoggingEventHandler));
    
    // 创建示例设备
    let devices = vec![
        AdvancedIoTDevice {
            id: "sensor_001".to_string(),
            name: "温度传感器1".to_string(),
            device_type: DeviceType::Sensor(SensorType::Temperature),
            status: DeviceStatus::Online { uptime: 3600, last_ping: 1640995200 },
            capabilities: vec![
                Capability {
                    name: "temperature_reading".to_string(),
                    version: "1.0".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "precision".to_string(),
                            value_type: ParameterType::Integer { min: 1, max: 3 },
                            default_value: ParameterValue::Integer(2),
                            constraints: Some(ParameterConstraints {
                                required: true,
                                read_only: false,
                                validation_rules: vec![ValidationRule::Range { min: 1.0, max: 3.0 }],
                            }),
                        }
                    ],
                    is_enabled: true,
                }
            ],
            metrics: DeviceMetrics {
                cpu_usage: 15.5,
                memory_usage: 45.2,
                network_usage: NetworkUsage {
                    bytes_sent: 1024,
                    bytes_received: 2048,
                    packets_sent: 10,
                    packets_received: 20,
                    connection_count: 1,
                },
                power_consumption: 2.5,
                temperature: 25.3,
                uptime: 3600,
            },
            last_updated: 1640995200,
        },
        AdvancedIoTDevice {
            id: "gateway_001".to_string(),
            name: "WiFi网关1".to_string(),
            device_type: DeviceType::Gateway(GatewayType::WiFi),
            status: DeviceStatus::Online { uptime: 7200, last_ping: 1640995200 },
            capabilities: vec![
                Capability {
                    name: "device_management".to_string(),
                    version: "2.0".to_string(),
                    parameters: vec![],
                    is_enabled: true,
                }
            ],
            metrics: DeviceMetrics {
                cpu_usage: 35.8,
                memory_usage: 68.4,
                network_usage: NetworkUsage {
                    bytes_sent: 10240,
                    bytes_received: 20480,
                    packets_sent: 100,
                    packets_received: 200,
                    connection_count: 5,
                },
                power_consumption: 8.2,
                temperature: 42.1,
                uptime: 7200,
            },
            last_updated: 1640995200,
        },
    ];
    
    // 添加设备
    for device in devices {
        device_manager.add_device(device).await?;
    }
    
    // 添加规则
    let mut rule_engine = RuleEngine::new();
    
    // 添加CPU使用率告警规则
    rule_engine.add_rule(Rule {
        id: "cpu_alert".to_string(),
        name: "CPU使用率告警".to_string(),
        condition: RuleCondition::MetricThreshold {
            device_id: "gateway_001".to_string(),
            metric_name: "cpu_usage".to_string(),
            operator: ComparisonOperator::GreaterThan,
            threshold: 80.0,
        },
        action: RuleAction::SendNotification {
            message: "网关CPU使用率过高！".to_string(),
            channels: vec![
                NotificationChannel::Email("admin@example.com".to_string()),
                NotificationChannel::Slack("#alerts".to_string()),
            ],
        },
        enabled: true,
        priority: 1,
    });
    
    // 添加设备离线告警规则
    rule_engine.add_rule(Rule {
        id: "device_offline_alert".to_string(),
        name: "设备离线告警".to_string(),
        condition: RuleCondition::DeviceStatus {
            device_id: "sensor_001".to_string(),
            status: DeviceStatus::Offline { last_seen: 0, reason: OfflineReason::Unknown },
        },
        action: RuleAction::LogEvent {
            level: LogLevel::Warning,
            message: "传感器设备离线".to_string(),
        },
        enabled: true,
        priority: 2,
    });
    
    // 模拟设备状态变化
    println!("\n模拟设备状态变化:");
    
    // 更新网关CPU使用率
    let high_cpu_metrics = DeviceMetrics {
        cpu_usage: 85.0, // 超过阈值
        memory_usage: 68.4,
        network_usage: NetworkUsage {
            bytes_sent: 10240,
            bytes_received: 20480,
            packets_sent: 100,
            packets_received: 200,
            connection_count: 5,
        },
        power_consumption: 8.2,
        temperature: 42.1,
        uptime: 7200,
    };
    
    device_manager.update_device_metrics("gateway_001", high_cpu_metrics).await?;
    
    // 模拟传感器离线
    device_manager.update_device_status("sensor_001", 
        DeviceStatus::Offline { 
            last_seen: 1640995200, 
            reason: OfflineReason::NetworkFailure 
        }).await?;
    
    // 获取设备统计
    let stats = device_manager.get_device_stats().await;
    println!("\n设备统计:");
    println!("  总设备数: {}", stats.total_devices);
    println!("  在线设备数: {}", stats.online_devices);
    println!("  离线设备数: {}", stats.offline_devices);
    println!("  错误设备数: {}", stats.error_devices);
    println!("  维护设备数: {}", stats.maintenance_devices);
    
    // 获取指标
    let metrics = device_manager.metrics_collector.get_metrics(Some("gateway_001")).await;
    println!("\n网关指标:");
    for metric in metrics {
        println!("  {}: {:.2}", metric.metric_name, metric.value);
    }
    
    // 执行规则
    println!("\n执行规则引擎:");
    device_manager.execute_rules().await?;
    
    println!("\n演示完成！");
    println!("本演示展示了Rust 1.90的高级特性在IoT开发中的应用：");
    println!("- 高级模式匹配用于状态变化检测");
    println!("- 改进的错误处理机制");
    println!("- 异步编程和并发安全");
    println!("- 类型安全的数据结构设计");
    println!("- 灵活的事件驱动架构");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_status_pattern_matching() {
        let online_status = DeviceStatus::Online { uptime: 1000, last_ping: 1640995200 };
        let offline_status = DeviceStatus::Offline { last_seen: 1640995200, reason: OfflineReason::NetworkFailure };
        
        match (&online_status, &offline_status) {
            (DeviceStatus::Online { .. }, DeviceStatus::Offline { .. }) => {
                assert!(true, "模式匹配成功");
            }
            _ => panic!("模式匹配失败"),
        }
    }

    #[test]
    fn test_parameter_validation() {
        let param = Parameter {
            name: "temperature".to_string(),
            value_type: ParameterType::Float { min: -50.0, max: 150.0 },
            default_value: ParameterValue::Float(25.0),
            constraints: Some(ParameterConstraints {
                required: true,
                read_only: false,
                validation_rules: vec![ValidationRule::Range { min: -50.0, max: 150.0 }],
            }),
        };
        
        assert_eq!(param.name, "temperature");
        assert!(matches!(param.value_type, ParameterType::Float { .. }));
    }

    #[tokio::test]
    async fn test_device_manager() {
        let device_manager = AdvancedIoTDeviceManager::new();
        let stats = device_manager.get_device_stats().await;
        assert_eq!(stats.total_devices, 0);
    }
}
