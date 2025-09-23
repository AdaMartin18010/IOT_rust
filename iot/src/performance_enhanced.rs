//! 增强的性能优化模块 - 基于 Rust 1.90 新特性
//! 
//! 本模块提供了利用 Rust 1.90 新特性的性能优化功能：
//! - 零成本抽象优化
//! - 改进的内存管理
//! - 高效的并发处理
//! - 智能缓存策略

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{RwLock, Semaphore};
use uuid::Uuid;

// 辅助函数：获取当前时间戳
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// 错误类型定义
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceError {
    #[error("性能监控初始化失败: {0}")]
    MonitorInitFailed(String),
    #[error("缓存操作失败: {0}")]
    CacheOperationFailed(String),
    #[error("内存分配失败: 需要 {required} 字节，可用 {available} 字节")]
    MemoryAllocationFailed { required: usize, available: usize },
    #[error("并发限制: 当前活跃任务 {current}，最大限制 {max}")]
    ConcurrencyLimitExceeded { current: usize, max: usize },
    #[error("性能阈值超限: {metric} = {value}，阈值 = {threshold}")]
    PerformanceThresholdExceeded { metric: String, value: f64, threshold: f64 },
}

// 智能性能指标类型 - 利用 Rust 1.90 的改进枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartPerformanceMetric {
    CpuUsage,           // CPU 使用率
    MemoryUsage,        // 内存使用率
    NetworkThroughput,  // 网络吞吐量
    ResponseTime,       // 响应时间
    ErrorRate,          // 错误率
    CacheHitRate,       // 缓存命中率
    TaskCompletionRate, // 任务完成率
    QueueDepth,         // 队列深度
}

// 性能阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    pub metric: SmartPerformanceMetric,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub enabled: bool,
}

// 性能数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: u64, // 使用u64时间戳替代Instant
    pub metric: SmartPerformanceMetric,
    pub value: f64,
    pub tags: HashMap<String, String>,
}

// 智能性能统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPerformanceStats {
    pub metric: SmartPerformanceMetric,
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub last_updated: u64, // 使用u64时间戳替代Instant
}

// 智能缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCacheConfig {
    pub max_size: usize,
    pub ttl: Duration,
    pub eviction_policy: EvictionPolicy,
    pub enable_compression: bool,
    pub enable_encryption: bool,
}

// 缓存淘汰策略 - 利用 Rust 1.90 的改进枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LRU,        // 最近最少使用
    LFU,        // 最少使用频率
    FIFO,       // 先进先出
    TTL,        // 基于时间
    Random,     // 随机淘汰
}

// 缓存项
#[derive(Debug, Clone)]
pub struct CacheItem<T> {
    pub key: String,
    pub value: T,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub size: usize,
}

// 智能缓存管理器
pub struct SmartCacheManager<T> {
    cache: Arc<RwLock<HashMap<String, CacheItem<T>>>>,
    config: SmartCacheConfig,
    stats: Arc<RwLock<SmartCacheStats>>,
    semaphore: Arc<Semaphore>,
}

#[derive(Debug)]
pub struct SmartCacheStats {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub evictions: AtomicU64,
    pub total_size: AtomicUsize,
    pub item_count: AtomicUsize,
}

impl<T: Clone + Send + Sync> SmartCacheManager<T> {
    pub fn new(config: SmartCacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(SmartCacheStats {
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
                evictions: AtomicU64::new(0),
                total_size: AtomicUsize::new(0),
                item_count: AtomicUsize::new(0),
            })),
            semaphore: Arc::new(Semaphore::new(100)), // 限制并发访问
        }
    }

    /// 获取缓存项
    pub async fn get(&self, key: &str) -> Option<T> {
        let _permit = self.semaphore.acquire().await.ok()?;
        
        let mut cache = self.cache.write().await;
        if let Some(item) = cache.get_mut(key) {
            // 更新访问统计
            item.last_accessed = Instant::now();
            item.access_count += 1;
            
            // 更新命中统计
            self.stats.read().await.hits.fetch_add(1, Ordering::Relaxed);
            
            Some(item.value.clone())
        } else {
            // 更新未命中统计
            self.stats.read().await.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// 设置缓存项
    pub async fn set(&self, key: String, value: T, size: usize) -> Result<(), PerformanceError> {
        let _permit = self.semaphore.acquire().await
            .map_err(|_| PerformanceError::ConcurrencyLimitExceeded { current: 100, max: 100 })?;
        
        let mut cache = self.cache.write().await;
        
        // 检查缓存大小限制
        if cache.len() >= self.config.max_size {
            self.evict_items(&mut cache).await?;
        }
        
        let item = CacheItem {
            key: key.clone(),
            value,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 1,
            size,
        };
        
        cache.insert(key, item);
        
        // 更新统计信息
        let stats = self.stats.write().await;
        stats.item_count.fetch_add(1, Ordering::Relaxed);
        stats.total_size.fetch_add(size, Ordering::Relaxed);
        
        Ok(())
    }

    /// 淘汰缓存项
    async fn evict_items(&self, cache: &mut HashMap<String, CacheItem<T>>) -> Result<(), PerformanceError> {
        let evict_count = cache.len() / 10; // 淘汰 10% 的项
        
        // 先收集需要删除的键，避免借用冲突
        let keys_to_remove: Vec<String> = match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                let mut items: Vec<_> = cache.iter().map(|(key, item)| (key.clone(), item.last_accessed)).collect();
                items.sort_by_key(|(_, last_accessed)| *last_accessed);
                items.iter().take(evict_count).map(|(key, _)| key.clone()).collect()
            },
            EvictionPolicy::LFU => {
                let mut items: Vec<_> = cache.iter().map(|(key, item)| (key.clone(), item.access_count)).collect();
                items.sort_by_key(|(_, access_count)| *access_count);
                items.iter().take(evict_count).map(|(key, _)| key.clone()).collect()
            },
            EvictionPolicy::TTL => {
                let now = Instant::now();
                cache.iter()
                    .filter(|(_, item)| now.duration_since(item.created_at) > self.config.ttl)
                    .map(|(key, _)| key.clone())
                    .take(evict_count)
                    .collect()
            },
            _ => {
                // 其他策略的简单实现
                cache.keys().take(evict_count).cloned().collect()
            },
        };
        
        // 现在删除收集到的键，避免借用冲突
        for key in keys_to_remove {
            if let Some(item) = cache.remove(&key) {
                let stats = self.stats.write().await;
                stats.evictions.fetch_add(1, Ordering::Relaxed);
                stats.total_size.fetch_sub(item.size, Ordering::Relaxed);
                stats.item_count.fetch_sub(1, Ordering::Relaxed);
            }
        }
        
        Ok(())
    }

    /// 获取缓存统计信息
    pub async fn get_stats(&self) -> SmartCacheStats {
        let stats_guard = self.stats.read().await;
        SmartCacheStats {
            hits: AtomicU64::new(stats_guard.hits.load(Ordering::Relaxed)),
            misses: AtomicU64::new(stats_guard.misses.load(Ordering::Relaxed)),
            evictions: AtomicU64::new(stats_guard.evictions.load(Ordering::Relaxed)),
            total_size: AtomicUsize::new(stats_guard.total_size.load(Ordering::Relaxed)),
            item_count: AtomicUsize::new(stats_guard.item_count.load(Ordering::Relaxed)),
        }
    }

    /// 计算缓存命中率
    pub async fn hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        let hits = stats.hits.load(Ordering::Relaxed);
        let misses = stats.misses.load(Ordering::Relaxed);
        
        if hits + misses == 0 {
            0.0
        } else {
            hits as f64 / (hits + misses) as f64 * 100.0
        }
    }
}

// 性能监控器
pub struct SmartPerformanceMonitor {
    metrics: Arc<RwLock<HashMap<SmartPerformanceMetric, Vec<PerformanceDataPoint>>>>,
    thresholds: Arc<RwLock<Vec<PerformanceThreshold>>>,
    stats: Arc<RwLock<HashMap<SmartPerformanceMetric, SmartPerformanceStats>>>,
    alerts: Arc<RwLock<Vec<PerformanceAlert>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: Uuid,
    pub metric: SmartPerformanceMetric,
    pub value: f64,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub timestamp: u64,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl SmartPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            thresholds: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 添加性能阈值
    pub async fn add_threshold(&self, threshold: PerformanceThreshold) {
        let mut thresholds = self.thresholds.write().await;
        thresholds.push(threshold);
    }

    /// 记录性能指标
    pub async fn record_metric(&self, data_point: PerformanceDataPoint) -> Result<(), PerformanceError> {
        // 存储数据点
        let mut metrics = self.metrics.write().await;
        metrics.entry(data_point.metric)
            .or_insert_with(Vec::new)
            .push(data_point.clone());
        
        // 保持最近 1000 个数据点
        if let Some(metric_data) = metrics.get_mut(&data_point.metric) {
            if metric_data.len() > 1000 {
                metric_data.drain(0..metric_data.len() - 1000);
            }
        }

        // 更新统计信息
        self.update_stats(data_point.metric).await?;

        // 检查阈值
        self.check_thresholds(&data_point).await?;

        Ok(())
    }

    /// 更新统计信息
    async fn update_stats(&self, metric: SmartPerformanceMetric) -> Result<(), PerformanceError> {
        let metrics = self.metrics.read().await;
        if let Some(data_points) = metrics.get(&metric) {
            if data_points.is_empty() {
                return Ok(());
            }

            let values: Vec<f64> = data_points.iter().map(|dp| dp.value).collect();
            let mut sorted_values = values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let count = values.len() as u64;
            let sum: f64 = values.iter().sum();
            let min = sorted_values.first().copied().unwrap_or(0.0);
            let max = sorted_values.last().copied().unwrap_or(0.0);
            let average = sum / count as f64;
            
            let p50_idx = (count as f64 * 0.5) as usize;
            let p95_idx = (count as f64 * 0.95) as usize;
            let p99_idx = (count as f64 * 0.99) as usize;
            
            let p50 = sorted_values.get(p50_idx).copied().unwrap_or(0.0);
            let p95 = sorted_values.get(p95_idx).copied().unwrap_or(0.0);
            let p99 = sorted_values.get(p99_idx).copied().unwrap_or(0.0);

            let stats = SmartPerformanceStats {
                metric,
                count,
                sum,
                min,
                max,
                average,
                p50,
                p95,
                p99,
                last_updated: current_timestamp(),
            };

            let mut stats_map = self.stats.write().await;
            stats_map.insert(metric, stats);
        }

        Ok(())
    }

    /// 检查性能阈值
    async fn check_thresholds(&self, data_point: &PerformanceDataPoint) -> Result<(), PerformanceError> {
        let thresholds = self.thresholds.read().await;
        
        for threshold in thresholds.iter() {
            if threshold.enabled && threshold.metric == data_point.metric {
                let severity = if data_point.value >= threshold.critical_threshold {
                    AlertSeverity::Critical
                } else if data_point.value >= threshold.warning_threshold {
                    AlertSeverity::Warning
                } else {
                    continue;
                };

                let alert = PerformanceAlert {
                    id: Uuid::new_v4(),
                    metric: data_point.metric,
                    value: data_point.value,
                    threshold: if severity == AlertSeverity::Critical {
                        threshold.critical_threshold
                    } else {
                        threshold.warning_threshold
                    },
                    severity,
                    timestamp: current_timestamp(),
                    message: format!(
                        "性能指标 {} 超过阈值: {:.2} >= {:.2}",
                        format!("{:?}", data_point.metric),
                        data_point.value,
                        threshold.warning_threshold
                    ),
                };

                let mut alerts = self.alerts.write().await;
                alerts.push(alert);
            }
        }

        Ok(())
    }

    /// 获取性能统计信息
    pub async fn get_stats(&self, metric: SmartPerformanceMetric) -> Option<SmartPerformanceStats> {
        let stats = self.stats.read().await;
        stats.get(&metric).cloned()
    }

    /// 获取所有性能统计信息
    pub async fn get_all_stats(&self) -> HashMap<SmartPerformanceMetric, SmartPerformanceStats> {
        self.stats.read().await.clone()
    }

    /// 获取性能告警
    pub async fn get_alerts(&self, limit: Option<usize>) -> Vec<PerformanceAlert> {
        let alerts = self.alerts.read().await;
        if let Some(limit) = limit {
            alerts.iter().rev().take(limit).cloned().collect()
        } else {
            alerts.clone()
        }
    }

    /// 清除告警
    pub async fn clear_alerts(&self) {
        let mut alerts = self.alerts.write().await;
        alerts.clear();
    }
}

// 并发任务管理器
pub struct ConcurrentTaskManager {
    semaphore: Arc<Semaphore>,
    active_tasks: Arc<AtomicUsize>,
    completed_tasks: Arc<AtomicU64>,
    failed_tasks: Arc<AtomicU64>,
    total_execution_time: Arc<AtomicU64>,
}

impl ConcurrentTaskManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            active_tasks: Arc::new(AtomicUsize::new(0)),
            completed_tasks: Arc::new(AtomicU64::new(0)),
            failed_tasks: Arc::new(AtomicU64::new(0)),
            total_execution_time: Arc::new(AtomicU64::new(0)),
        }
    }

    /// 执行任务
    pub async fn execute_task<F, R>(&self, task: F) -> Result<R, PerformanceError>
    where
        F: FnOnce() -> Result<R, Box<dyn std::error::Error + Send + Sync>>,
    {
        let _permit = self.semaphore.acquire().await
            .map_err(|_| PerformanceError::ConcurrencyLimitExceeded {
                current: self.active_tasks.load(Ordering::Relaxed),
                max: self.semaphore.available_permits(),
            })?;

        self.active_tasks.fetch_add(1, Ordering::Relaxed);
        let start_time = Instant::now();

        let result = task();

        let execution_time = start_time.elapsed();
        self.total_execution_time.fetch_add(execution_time.as_nanos() as u64, Ordering::Relaxed);
        self.active_tasks.fetch_sub(1, Ordering::Relaxed);

        match result {
            Ok(value) => {
                self.completed_tasks.fetch_add(1, Ordering::Relaxed);
                Ok(value)
            },
            Err(_) => {
                self.failed_tasks.fetch_add(1, Ordering::Relaxed);
                Err(PerformanceError::PerformanceThresholdExceeded {
                    metric: "task_execution".to_string(),
                    value: 1.0,
                    threshold: 0.0,
                })
            },
        }
    }

    /// 获取任务统计信息
    pub fn get_task_stats(&self) -> TaskStats {
        let active = self.active_tasks.load(Ordering::Relaxed);
        let completed = self.completed_tasks.load(Ordering::Relaxed);
        let failed = self.failed_tasks.load(Ordering::Relaxed);
        let total_time = self.total_execution_time.load(Ordering::Relaxed);
        
        let success_rate = if completed + failed > 0 {
            (completed as f64 / (completed + failed) as f64) * 100.0
        } else {
            0.0
        };

        let avg_execution_time = if completed > 0 {
            Duration::from_nanos(total_time / completed)
        } else {
            Duration::ZERO
        };

        TaskStats {
            active_tasks: active,
            completed_tasks: completed,
            failed_tasks: failed,
            success_rate,
            average_execution_time: avg_execution_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub active_tasks: usize,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub success_rate: f64,
    pub average_execution_time: Duration,
}

// 默认实现
impl Default for SmartPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = SmartCacheManager::new(SmartCacheConfig {
            max_size: 10,
            ttl: Duration::from_secs(60),
            eviction_policy: EvictionPolicy::LRU,
            enable_compression: false,
            enable_encryption: false,
        });

        // 测试设置和获取
        cache.set("key1".to_string(), "value1".to_string(), 100).await.unwrap();
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));

        // 测试未命中
        let value = cache.get("nonexistent").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_performance_monitor() {
        let monitor = SmartPerformanceMonitor::new();

        // 添加阈值
        let threshold = PerformanceThreshold {
            metric: SmartPerformanceMetric::CpuUsage,
            warning_threshold: 80.0,
            critical_threshold: 95.0,
            enabled: true,
        };
        monitor.add_threshold(threshold).await;

        // 记录指标
        let data_point = PerformanceDataPoint {
            timestamp: current_timestamp(),
            metric: SmartPerformanceMetric::CpuUsage,
            value: 85.0,
            tags: HashMap::new(),
        };
        monitor.record_metric(data_point).await.unwrap();

        // 检查告警
        let alerts = monitor.get_alerts(Some(10)).await;
        assert!(!alerts.is_empty());
    }

    #[tokio::test]
    async fn test_concurrent_task_manager() {
        let manager = ConcurrentTaskManager::new(2);

        // 执行任务
        let result = manager.execute_task(|| {
            Ok::<String, Box<dyn std::error::Error + Send + Sync>>("success".to_string())
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");

        let stats = manager.get_task_stats();
        assert_eq!(stats.completed_tasks, 1);
        assert_eq!(stats.failed_tasks, 0);
    }
}
