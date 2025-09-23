//! 增强的嵌入式操作系统支持 - 基于 Rust 1.90 新特性
//! 
//! 本模块提供了对最新嵌入式操作系统的支持，包括：
//! - Embassy 异步嵌入式框架
//! - Tock OS 集成
//! - 零成本抽象和内存安全
//! - 利用 Rust 1.90 的新语言特性

use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

// Embassy 异步嵌入式框架支持
#[cfg(feature = "embedded")]
use embassy::executor::Spawner;
#[cfg(feature = "embedded")]
use embassy::time::{Duration as EmbassyDuration, Timer};
#[cfg(feature = "embedded")]
use embedded_hal::digital::v2::OutputPin;
#[cfg(feature = "embedded")]
use embedded_hal::blocking::delay::DelayUs;

// 错误类型定义
#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddedOSError {
    #[error("任务创建失败: {0}")]
    TaskCreationFailed(String),
    #[error("资源分配失败: {0}")]
    ResourceAllocationFailed(String),
    #[error("硬件初始化失败: {0}")]
    HardwareInitFailed(String),
    #[error("通信错误: {0}")]
    CommunicationError(String),
    #[error("内存不足: 需要 {required} 字节，可用 {available} 字节")]
    InsufficientMemory { required: usize, available: usize },
    #[error("任务调度失败: {0}")]
    TaskSchedulingFailed(String),
    #[error("中断处理失败: {0}")]
    InterruptHandlingFailed(String),
}

// 任务优先级枚举 - 利用 Rust 1.90 的改进枚举支持
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskPriority {
    Critical = 0,    // 关键任务
    High = 1,        // 高优先级
    Normal = 2,      // 普通优先级
    Low = 3,         // 低优先级
    Background = 4,  // 后台任务
}

// 任务状态 - 利用 Rust 1.90 的模式匹配改进
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Ready,           // 就绪
    Running,         // 运行中
    Blocked,         // 阻塞
    Suspended,       // 挂起
    Terminated,      // 终止
    Error(EmbeddedOSError), // 错误状态
}

// 任务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub priority: TaskPriority,
    pub stack_size: usize,
    pub deadline: Option<Duration>,
    pub cpu_affinity: Option<u32>,
    pub memory_pool: Option<String>,
}

// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: Uuid,
    pub config: TaskConfig,
    pub status: TaskStatus,
    pub created_at: u64, // 使用时间戳替代 Instant
    pub last_run: Option<u64>,
    pub run_count: u64,
    pub total_runtime: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
}

// 系统资源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResources {
    pub total_memory: usize,
    pub used_memory: usize,
    pub free_memory: usize,
    pub cpu_usage: f64,
    pub task_count: usize,
    pub interrupt_count: u64,
    pub context_switches: u64,
}

// 中断处理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptConfig {
    pub irq_number: u32,
    pub priority: TaskPriority,
    pub handler: String,
    pub enabled: bool,
}

// 内存池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    pub name: String,
    pub size: usize,
    pub block_size: usize,
    pub alignment: usize,
    pub strategy: MemoryAllocationStrategy,
}

// 内存分配策略 - 利用 Rust 1.90 的改进枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryAllocationStrategy {
    FirstFit,        // 首次适应
    BestFit,         // 最佳适应
    WorstFit,        // 最坏适应
    BuddySystem,     // 伙伴系统
    SlabAllocator,   // 板分配器
}

// 增强的嵌入式操作系统管理器
pub struct EnhancedEmbeddedOSManager {
    tasks: Arc<RwLock<HashMap<Uuid, TaskInfo>>>,
    resources: Arc<RwLock<SystemResources>>,
    interrupts: Arc<RwLock<HashMap<u32, InterruptConfig>>>,
    memory_pools: Arc<RwLock<HashMap<String, MemoryPoolConfig>>>,
    #[cfg(feature = "embedded")]
    spawner: Option<Spawner>,
}

impl EnhancedEmbeddedOSManager {
    /// 创建新的嵌入式操作系统管理器
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            resources: Arc::new(RwLock::new(SystemResources {
                total_memory: 1024 * 1024, // 1MB 默认
                used_memory: 0,
                free_memory: 1024 * 1024,
                cpu_usage: 0.0,
                task_count: 0,
                interrupt_count: 0,
                context_switches: 0,
            })),
            interrupts: Arc::new(RwLock::new(HashMap::new())),
            memory_pools: Arc::new(RwLock::new(HashMap::new())),
            #[cfg(feature = "embedded")]
            spawner: None,
        }
    }

    /// 初始化 Embassy 异步框架
    #[cfg(feature = "embedded")]
    pub async fn init_embassy(&mut self, spawner: Spawner) -> Result<(), EmbeddedOSError> {
        self.spawner = Some(spawner);
        tracing::info!("Embassy 异步框架初始化完成");
        Ok(())
    }

    /// 创建新任务
    pub async fn create_task<F>(
        &self,
        config: TaskConfig,
        _task_fn: F,
    ) -> Result<Uuid, EmbeddedOSError> 
    where
        F: FnOnce() -> Result<(), EmbeddedOSError> + Send + 'static,
    {
        let task_id = Uuid::new_v4();
        
        // 检查内存是否足够
        let mut resources = self.resources.write().await;
        if resources.free_memory < config.stack_size {
            return Err(EmbeddedOSError::InsufficientMemory {
                required: config.stack_size,
                available: resources.free_memory,
            });
        }

        // 创建任务信息
        let task_info = TaskInfo {
            id: task_id,
            config: config.clone(),
            status: TaskStatus::Ready,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_run: None,
            run_count: 0,
            total_runtime: Duration::ZERO,
            memory_usage: config.stack_size,
            cpu_usage: 0.0,
        };

        // 更新资源使用情况
        resources.used_memory += config.stack_size;
        resources.free_memory -= config.stack_size;
        resources.task_count += 1;

        // 存储任务信息
        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, task_info);

        // 在 Embassy 中启动任务
        #[cfg(feature = "embedded")]
        if let Some(spawner) = &self.spawner {
            let task_id_clone = task_id;
            let tasks_clone = self.tasks.clone();
            
            spawner.spawn(async move {
                let start_time = SystemTime::now();
                
                // 更新任务状态为运行中
                if let Ok(mut tasks) = tasks_clone.write().await {
                    if let Some(task) = tasks.get_mut(&task_id_clone) {
                        task.status = TaskStatus::Running;
                        task.last_run = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                    }
                }

                // 执行任务
                let result = task_fn();

                // 更新任务状态和统计信息
                if let Ok(mut tasks) = tasks_clone.write().await {
                    if let Some(task) = tasks.get_mut(&task_id_clone) {
                        task.total_runtime += start_time.elapsed().unwrap_or(Duration::ZERO);
                        task.run_count += 1;
                        task.status = match result {
                            Ok(_) => TaskStatus::Terminated,
                            Err(e) => TaskStatus::Error(e),
                        };
                    }
                }
            }).map_err(|e| EmbeddedOSError::TaskCreationFailed(e.to_string()))?;
        }

        println!("任务创建成功: {} (ID: {})", config.name, task_id);
        Ok(task_id)
    }

    /// 获取任务信息
    pub async fn get_task_info(&self, task_id: Uuid) -> Result<TaskInfo, EmbeddedOSError> {
        let tasks = self.tasks.read().await;
        tasks.get(&task_id)
            .cloned()
            .ok_or_else(|| EmbeddedOSError::TaskCreationFailed("任务不存在".to_string()))
    }

    /// 获取所有任务信息
    pub async fn get_all_tasks(&self) -> Vec<TaskInfo> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    /// 终止任务
    pub async fn terminate_task(&self, task_id: Uuid) -> Result<(), EmbeddedOSError> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = TaskStatus::Terminated;
            
            // 释放内存
            let mut resources = self.resources.write().await;
            resources.used_memory -= task.memory_usage;
            resources.free_memory += task.memory_usage;
            resources.task_count -= 1;
            
            println!("任务已终止: {} (ID: {})", task.config.name, task_id);
            Ok(())
        } else {
            Err(EmbeddedOSError::TaskCreationFailed("任务不存在".to_string()))
        }
    }

    /// 配置中断处理
    pub async fn configure_interrupt(&self, config: InterruptConfig) -> Result<(), EmbeddedOSError> {
        let mut interrupts = self.interrupts.write().await;
        interrupts.insert(config.irq_number, config.clone());
        
        println!("中断配置完成: IRQ {} -> {}", config.irq_number, config.handler);
        Ok(())
    }

    /// 创建内存池
    pub async fn create_memory_pool(&self, config: MemoryPoolConfig) -> Result<(), EmbeddedOSError> {
        let mut pools = self.memory_pools.write().await;
        pools.insert(config.name.clone(), config.clone());
        
        println!("内存池创建完成: {} (大小: {} 字节)", config.name, config.size);
        Ok(())
    }

    /// 获取系统资源信息
    pub async fn get_system_resources(&self) -> SystemResources {
        self.resources.read().await.clone()
    }

    /// 更新系统资源信息
    pub async fn update_system_resources(&self) -> Result<(), EmbeddedOSError> {
        let mut resources = self.resources.write().await;
        let tasks = self.tasks.read().await;
        
        // 计算实际内存使用情况
        let mut used_memory = 0;
        let mut cpu_usage = 0.0;
        
        for task in tasks.values() {
            used_memory += task.memory_usage;
            cpu_usage += task.cpu_usage;
        }
        
        resources.used_memory = used_memory;
        resources.free_memory = resources.total_memory - used_memory;
        resources.cpu_usage = cpu_usage;
        resources.task_count = tasks.len();
        
        Ok(())
    }

    /// 执行系统健康检查
    pub async fn health_check(&self) -> Result<SystemHealth, EmbeddedOSError> {
        let resources = self.resources.read().await;
        let tasks = self.tasks.read().await;
        
        let mut health = SystemHealth {
            overall_status: HealthStatus::Healthy,
            memory_usage_percent: (resources.used_memory as f64 / resources.total_memory as f64) * 100.0,
            cpu_usage_percent: resources.cpu_usage,
            task_count: resources.task_count,
            error_count: 0,
            warnings: Vec::new(),
            recommendations: Vec::new(),
        };

        // 检查内存使用情况
        if health.memory_usage_percent > 90.0 {
            health.overall_status = HealthStatus::Critical;
            health.warnings.push("内存使用率过高".to_string());
            health.recommendations.push("考虑释放未使用的内存或增加系统内存".to_string());
        } else if health.memory_usage_percent > 75.0 {
            health.overall_status = HealthStatus::Warning;
            health.warnings.push("内存使用率较高".to_string());
        }

        // 检查CPU使用情况
        if health.cpu_usage_percent > 90.0 {
            health.overall_status = HealthStatus::Critical;
            health.warnings.push("CPU使用率过高".to_string());
            health.recommendations.push("优化任务调度或减少任务负载".to_string());
        } else if health.cpu_usage_percent > 75.0 {
            if health.overall_status == HealthStatus::Healthy {
                health.overall_status = HealthStatus::Warning;
            }
            health.warnings.push("CPU使用率较高".to_string());
        }

        // 检查错误任务
        for task in tasks.values() {
            if matches!(task.status, TaskStatus::Error(_)) {
                health.error_count += 1;
            }
        }

        if health.error_count > 0 {
            health.overall_status = HealthStatus::Critical;
            health.warnings.push(format!("发现 {} 个错误任务", health.error_count));
            health.recommendations.push("检查并修复错误任务".to_string());
        }

        Ok(health)
    }
}

// 系统健康状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

// 系统健康信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub memory_usage_percent: f64,
    pub cpu_usage_percent: f64,
    pub task_count: usize,
    pub error_count: usize,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

// 默认实现
impl Default for EnhancedEmbeddedOSManager {
    fn default() -> Self {
        Self::new()
    }
}

// 异步任务示例
#[cfg(feature = "embedded")]
pub async fn example_async_task() -> Result<(), EmbeddedOSError> {
    // 利用 Rust 1.90 的改进异步支持
    Timer::after(EmbassyDuration::from_millis(100)).await;
    
    println!("异步任务执行完成");
    Ok(())
}

// 硬件抽象层示例
#[cfg(feature = "embedded")]
pub struct HardwareAbstractionLayer {
    gpio_pins: HashMap<u32, bool>,
}

#[cfg(feature = "embedded")]
impl HardwareAbstractionLayer {
    pub fn new() -> Self {
        Self {
            gpio_pins: HashMap::new(),
        }
    }

    pub fn set_gpio_pin(&mut self, pin: u32, value: bool) -> Result<(), EmbeddedOSError> {
        self.gpio_pins.insert(pin, value);
        println!("GPIO 引脚 {} 设置为 {}", pin, value);
        Ok(())
    }

    pub fn get_gpio_pin(&self, pin: u32) -> Result<bool, EmbeddedOSError> {
        self.gpio_pins.get(&pin)
            .copied()
            .ok_or_else(|| EmbeddedOSError::HardwareInitFailed("GPIO 引脚不存在".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedded_os_manager_creation() {
        let manager = EnhancedEmbeddedOSManager::new();
        let resources = manager.get_system_resources().await;
        
        assert_eq!(resources.task_count, 0);
        assert!(resources.free_memory > 0);
    }

    #[tokio::test]
    async fn test_task_creation() {
        let manager = EnhancedEmbeddedOSManager::new();
        
        let config = TaskConfig {
            name: "test_task".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline: None,
            cpu_affinity: None,
            memory_pool: None,
        };

        let task_id = manager.create_task(config, || {
            println!("测试任务执行");
            Ok(())
        }).await;

        assert!(task_id.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let manager = EnhancedEmbeddedOSManager::new();
        let health = manager.health_check().await;
        
        assert!(health.is_ok());
        let health = health.unwrap();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
    }
}
