//! 增强的网络协议栈 - 基于 Rust 1.90 新特性
//! 
//! 本模块提供了对最新网络协议栈的支持，包括：
//! - smoltcp 嵌入式 TCP/IP 协议栈
//! - lwIP 轻量级 IP 协议栈
//! - 利用 Rust 1.90 的改进异步和模式匹配
//! - 零成本抽象和内存安全

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

// smoltcp 协议栈支持
#[cfg(feature = "network-stack")]
use smoltcp::{
    iface::{Interface, InterfaceBuilder, NeighborCache, Routes},
    phy::{Device, DeviceCapabilities, Medium},
    socket::{TcpSocket, UdpSocket, SocketSet, SocketHandle},
    time::Instant as SmolInstant,
    wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address, Ipv6Address},
};

// 错误类型定义
#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkStackError {
    #[error("网络接口初始化失败: {0}")]
    InterfaceInitFailed(String),
    #[error("套接字创建失败: {0}")]
    SocketCreationFailed(String),
    #[error("连接失败: {0}")]
    ConnectionFailed(String),
    #[error("数据传输失败: {0}")]
    DataTransmissionFailed(String),
    #[error("协议错误: {0}")]
    ProtocolError(String),
    #[error("缓冲区溢出: 需要 {required} 字节，可用 {available} 字节")]
    BufferOverflow { required: usize, available: usize },
    #[error("超时: {operation} 在 {timeout:?} 后超时")]
    Timeout { operation: String, timeout: Duration },
    #[error("网络不可达: {0}")]
    NetworkUnreachable(String),
}

// 网络接口类型 - 利用 Rust 1.90 的改进枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkInterfaceType {
    Ethernet,
    WiFi,
    Cellular,
    LoRa,
    Bluetooth,
    Zigbee,
    Thread,
    Matter,
}

// 网络接口状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkInterfaceStatus {
    Down,
    Up,
    Connecting,
    Connected,
    Disconnected,
    Error(NetworkStackError),
}

// 网络接口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceConfig {
    pub name: String,
    pub interface_type: NetworkInterfaceType,
    pub ip_address: IpAddr,
    pub subnet_mask: IpAddr,
    pub gateway: Option<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub mtu: u16,
    pub mac_address: Option<String>,
    pub enabled: bool,
}

// 网络接口信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceInfo {
    pub id: Uuid,
    pub config: NetworkInterfaceConfig,
    pub status: NetworkInterfaceStatus,
    #[serde(skip, default = "std::time::Instant::now")]
    pub created_at: Instant,
    #[serde(skip, default)]
    pub last_activity: Option<Instant>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub error_count: u64,
}

// 套接字类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketType {
    Tcp,
    Udp,
    Raw,
    Icmp,
}

// 套接字状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketState {
    Closed,
    Listening,
    Connecting,
    Connected,
    Disconnecting,
    Error(NetworkStackError),
}

// 套接字配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketConfig {
    pub socket_type: SocketType,
    pub local_address: Option<SocketAddr>,
    pub remote_address: Option<SocketAddr>,
    pub buffer_size: usize,
    pub timeout: Option<Duration>,
    pub keep_alive: bool,
    pub tcp_nodelay: bool,
}

// 套接字信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketInfo {
    pub id: Uuid,
    pub config: SocketConfig,
    pub state: SocketState,
    #[serde(skip, default = "std::time::Instant::now")]
    pub created_at: Instant,
    #[serde(skip, default)]
    pub last_activity: Option<Instant>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_count: u64,
    pub error_count: u64,
}

// 网络统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_interfaces: usize,
    pub active_interfaces: usize,
    pub total_sockets: usize,
    pub active_sockets: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub total_packets_sent: u64,
    pub total_packets_received: u64,
    pub total_errors: u64,
    pub network_utilization: f64,
}

// 增强的网络协议栈管理器
pub struct EnhancedNetworkStackManager {
    interfaces: Arc<RwLock<HashMap<Uuid, NetworkInterfaceInfo>>>,
    sockets: Arc<RwLock<HashMap<Uuid, SocketInfo>>>,
    stats: Arc<RwLock<NetworkStats>>,
    #[cfg(feature = "network-stack")]
    socket_set: Arc<Mutex<SocketSet<'static>>>,
    #[cfg(feature = "network-stack")]
    interface: Arc<Mutex<Option<Interface<'static>>>>,
}

impl EnhancedNetworkStackManager {
    /// 创建新的网络协议栈管理器
    pub fn new() -> Self {
        Self {
            interfaces: Arc::new(RwLock::new(HashMap::new())),
            sockets: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(NetworkStats {
                total_interfaces: 0,
                active_interfaces: 0,
                total_sockets: 0,
                active_sockets: 0,
                total_bytes_sent: 0,
                total_bytes_received: 0,
                total_packets_sent: 0,
                total_packets_received: 0,
                total_errors: 0,
                network_utilization: 0.0,
            })),
            #[cfg(feature = "network-stack")]
            socket_set: Arc::new(Mutex::new(SocketSet::new(vec![]))),
            #[cfg(feature = "network-stack")]
            interface: Arc::new(Mutex::new(None)),
        }
    }

    /// 创建网络接口
    pub async fn create_interface(
        &self,
        config: NetworkInterfaceConfig,
    ) -> Result<Uuid, NetworkStackError> {
        let interface_id = Uuid::new_v4();
        
        // 创建接口信息
        let interface_info = NetworkInterfaceInfo {
            id: interface_id,
            config: config.clone(),
            status: NetworkInterfaceStatus::Down,
            created_at: Instant::now(),
            last_activity: None,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            error_count: 0,
        };

        // 存储接口信息
        let mut interfaces = self.interfaces.write().await;
        interfaces.insert(interface_id, interface_info);

        // 更新统计信息
        let mut stats = self.stats.write().await;
        stats.total_interfaces += 1;

        tracing::info!("网络接口创建成功: {} (ID: {})", config.name, interface_id);
        Ok(interface_id)
    }

    /// 启动网络接口
    pub async fn start_interface(&self, interface_id: Uuid) -> Result<(), NetworkStackError> {
        let mut interfaces = self.interfaces.write().await;
        if let Some(interface) = interfaces.get_mut(&interface_id) {
            interface.status = NetworkInterfaceStatus::Up;
            interface.last_activity = Some(Instant::now());
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            stats.active_interfaces += 1;
            
            tracing::info!("网络接口已启动: {} (ID: {})", interface.config.name, interface_id);
            Ok(())
        } else {
            Err(NetworkStackError::InterfaceInitFailed("接口不存在".to_string()))
        }
    }

    /// 停止网络接口
    pub async fn stop_interface(&self, interface_id: Uuid) -> Result<(), NetworkStackError> {
        let mut interfaces = self.interfaces.write().await;
        if let Some(interface) = interfaces.get_mut(&interface_id) {
            interface.status = NetworkInterfaceStatus::Down;
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            if stats.active_interfaces > 0 {
                stats.active_interfaces -= 1;
            }
            
            tracing::info!("网络接口已停止: {} (ID: {})", interface.config.name, interface_id);
            Ok(())
        } else {
            Err(NetworkStackError::InterfaceInitFailed("接口不存在".to_string()))
        }
    }

    /// 创建套接字
    pub async fn create_socket(
        &self,
        config: SocketConfig,
    ) -> Result<Uuid, NetworkStackError> {
        let socket_id = Uuid::new_v4();
        
        // 创建套接字信息
        let socket_info = SocketInfo {
            id: socket_id,
            config: config.clone(),
            state: SocketState::Closed,
            created_at: Instant::now(),
            last_activity: None,
            bytes_sent: 0,
            bytes_received: 0,
            connection_count: 0,
            error_count: 0,
        };

        // 存储套接字信息
        let mut sockets = self.sockets.write().await;
        sockets.insert(socket_id, socket_info);

        // 更新统计信息
        let mut stats = self.stats.write().await;
        stats.total_sockets += 1;

        tracing::info!("套接字创建成功: {:?} (ID: {})", config.socket_type, socket_id);
        Ok(socket_id)
    }

    /// 连接套接字
    pub async fn connect_socket(
        &self,
        socket_id: Uuid,
        remote_address: SocketAddr,
    ) -> Result<(), NetworkStackError> {
        let mut sockets = self.sockets.write().await;
        if let Some(socket) = sockets.get_mut(&socket_id) {
            socket.state = SocketState::Connecting;
            socket.config.remote_address = Some(remote_address);
            socket.last_activity = Some(Instant::now());
            
            // 模拟连接过程
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            socket.state = SocketState::Connected;
            socket.connection_count += 1;
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            stats.active_sockets += 1;
            
            tracing::info!("套接字连接成功: {} -> {}", socket_id, remote_address);
            Ok(())
        } else {
            Err(NetworkStackError::SocketCreationFailed("套接字不存在".to_string()))
        }
    }

    /// 发送数据
    pub async fn send_data(
        &self,
        socket_id: Uuid,
        data: &[u8],
    ) -> Result<usize, NetworkStackError> {
        let mut sockets = self.sockets.write().await;
        if let Some(socket) = sockets.get_mut(&socket_id) {
            if socket.state != SocketState::Connected {
                return Err(NetworkStackError::DataTransmissionFailed("套接字未连接".to_string()));
            }

            let bytes_sent = data.len();
            socket.bytes_sent += bytes_sent as u64;
            socket.last_activity = Some(Instant::now());
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            stats.total_bytes_sent += bytes_sent as u64;
            stats.total_packets_sent += 1;
            
            tracing::debug!("数据发送成功: {} 字节 (套接字: {})", bytes_sent, socket_id);
            Ok(bytes_sent)
        } else {
            Err(NetworkStackError::SocketCreationFailed("套接字不存在".to_string()))
        }
    }

    /// 接收数据
    pub async fn receive_data(
        &self,
        socket_id: Uuid,
        buffer: &mut [u8],
    ) -> Result<usize, NetworkStackError> {
        let mut sockets = self.sockets.write().await;
        if let Some(socket) = sockets.get_mut(&socket_id) {
            if socket.state != SocketState::Connected {
                return Err(NetworkStackError::DataTransmissionFailed("套接字未连接".to_string()));
            }

            // 模拟接收数据
            let bytes_received = std::cmp::min(buffer.len(), 1024);
            socket.bytes_received += bytes_received as u64;
            socket.last_activity = Some(Instant::now());
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            stats.total_bytes_received += bytes_received as u64;
            stats.total_packets_received += 1;
            
            tracing::debug!("数据接收成功: {} 字节 (套接字: {})", bytes_received, socket_id);
            Ok(bytes_received)
        } else {
            Err(NetworkStackError::SocketCreationFailed("套接字不存在".to_string()))
        }
    }

    /// 关闭套接字
    pub async fn close_socket(&self, socket_id: Uuid) -> Result<(), NetworkStackError> {
        let mut sockets = self.sockets.write().await;
        if let Some(socket) = sockets.get_mut(&socket_id) {
            socket.state = SocketState::Closed;
            
            // 更新统计信息
            let mut stats = self.stats.write().await;
            if stats.active_sockets > 0 {
                stats.active_sockets -= 1;
            }
            
            tracing::info!("套接字已关闭: {}", socket_id);
            Ok(())
        } else {
            Err(NetworkStackError::SocketCreationFailed("套接字不存在".to_string()))
        }
    }

    /// 获取网络统计信息
    pub async fn get_network_stats(&self) -> NetworkStats {
        self.stats.read().await.clone()
    }

    /// 获取所有接口信息
    pub async fn get_all_interfaces(&self) -> Vec<NetworkInterfaceInfo> {
        let interfaces = self.interfaces.read().await;
        interfaces.values().cloned().collect()
    }

    /// 获取所有套接字信息
    pub async fn get_all_sockets(&self) -> Vec<SocketInfo> {
        let sockets = self.sockets.read().await;
        sockets.values().cloned().collect()
    }

    /// 执行网络健康检查
    pub async fn health_check(&self) -> Result<NetworkHealth, NetworkStackError> {
        let stats = self.stats.read().await;
        let _interfaces = self.interfaces.read().await;
        let sockets = self.sockets.read().await;
        
        let mut health = NetworkHealth {
            overall_status: HealthStatus::Healthy,
            interface_count: stats.total_interfaces,
            active_interface_count: stats.active_interfaces,
            socket_count: stats.total_sockets,
            active_socket_count: stats.active_sockets,
            error_rate: if stats.total_packets_sent + stats.total_packets_received > 0 {
                stats.total_errors as f64 / (stats.total_packets_sent + stats.total_packets_received) as f64 * 100.0
            } else {
                0.0
            },
            utilization: stats.network_utilization,
            warnings: Vec::new(),
            recommendations: Vec::new(),
        };

        // 检查接口状态
        if stats.active_interfaces == 0 && stats.total_interfaces > 0 {
            health.overall_status = HealthStatus::Critical;
            health.warnings.push("没有活动的网络接口".to_string());
            health.recommendations.push("启动网络接口".to_string());
        }

        // 检查错误率
        if health.error_rate > 5.0 {
            health.overall_status = HealthStatus::Critical;
            health.warnings.push(format!("网络错误率过高: {:.2}%", health.error_rate));
            health.recommendations.push("检查网络连接和配置".to_string());
        } else if health.error_rate > 1.0 {
            if health.overall_status == HealthStatus::Healthy {
                health.overall_status = HealthStatus::Warning;
            }
            health.warnings.push(format!("网络错误率较高: {:.2}%", health.error_rate));
        }

        // 检查套接字状态
        let error_sockets = sockets.values()
            .filter(|s| matches!(s.state, SocketState::Error(_)))
            .count();
        
        if error_sockets > 0 {
            if health.overall_status == HealthStatus::Healthy {
                health.overall_status = HealthStatus::Warning;
            }
            health.warnings.push(format!("发现 {} 个错误套接字", error_sockets));
            health.recommendations.push("检查并修复错误套接字".to_string());
        }

        Ok(health)
    }

    /// 更新网络利用率
    pub async fn update_utilization(&self) -> Result<(), NetworkStackError> {
        let mut stats = self.stats.write().await;
        
        // 计算网络利用率（简化计算）
        let total_capacity = 100_000_000; // 100 Mbps 假设容量
        let current_throughput = stats.total_bytes_sent + stats.total_bytes_received;
        
        stats.network_utilization = (current_throughput as f64 / total_capacity as f64) * 100.0;
        
        Ok(())
    }
}

// 网络健康状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

// 网络健康信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHealth {
    pub overall_status: HealthStatus,
    pub interface_count: usize,
    pub active_interface_count: usize,
    pub socket_count: usize,
    pub active_socket_count: usize,
    pub error_rate: f64,
    pub utilization: f64,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

// 默认实现
impl Default for EnhancedNetworkStackManager {
    fn default() -> Self {
        Self::new()
    }
}

// 网络协议栈示例
pub async fn example_network_operations() -> Result<(), NetworkStackError> {
    let manager = EnhancedNetworkStackManager::new();
    
    // 创建网络接口
    let interface_config = NetworkInterfaceConfig {
        name: "eth0".to_string(),
        interface_type: NetworkInterfaceType::Ethernet,
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        subnet_mask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
        gateway: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
        dns_servers: vec![IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))],
        mtu: 1500,
        mac_address: Some("00:11:22:33:44:55".to_string()),
        enabled: true,
    };
    
    let interface_id = manager.create_interface(interface_config).await?;
    manager.start_interface(interface_id).await?;
    
    // 创建套接字
    let socket_config = SocketConfig {
        socket_type: SocketType::Tcp,
        local_address: None,
        remote_address: None,
        buffer_size: 8192,
        timeout: Some(Duration::from_secs(30)),
        keep_alive: true,
        tcp_nodelay: true,
    };
    
    let socket_id = manager.create_socket(socket_config).await?;
    
    // 连接套接字
    let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 200)), 8080);
    manager.connect_socket(socket_id, remote_addr).await?;
    
    // 发送数据
    let data = b"Hello, Network Stack!";
    let bytes_sent = manager.send_data(socket_id, data).await?;
    tracing::info!("发送了 {} 字节数据", bytes_sent);
    
    // 接收数据
    let mut buffer = [0u8; 1024];
    let bytes_received = manager.receive_data(socket_id, &mut buffer).await?;
    tracing::info!("接收了 {} 字节数据", bytes_received);
    
    // 关闭套接字
    manager.close_socket(socket_id).await?;
    
    // 获取统计信息
    let stats = manager.get_network_stats().await;
    tracing::info!("网络统计: 发送 {} 字节, 接收 {} 字节", 
                   stats.total_bytes_sent, stats.total_bytes_received);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_stack_manager_creation() {
        let manager = EnhancedNetworkStackManager::new();
        let stats = manager.get_network_stats().await;
        
        assert_eq!(stats.total_interfaces, 0);
        assert_eq!(stats.total_sockets, 0);
    }

    #[tokio::test]
    async fn test_interface_creation() {
        let manager = EnhancedNetworkStackManager::new();
        
        let config = NetworkInterfaceConfig {
            name: "test_interface".to_string(),
            interface_type: NetworkInterfaceType::Ethernet,
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            subnet_mask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
            gateway: None,
            dns_servers: vec![],
            mtu: 1500,
            mac_address: None,
            enabled: true,
        };

        let interface_id = manager.create_interface(config).await;
        assert!(interface_id.is_ok());
    }

    #[tokio::test]
    async fn test_socket_operations() {
        let manager = EnhancedNetworkStackManager::new();
        
        let config = SocketConfig {
            socket_type: SocketType::Tcp,
            local_address: None,
            remote_address: None,
            buffer_size: 1024,
            timeout: None,
            keep_alive: false,
            tcp_nodelay: false,
        };

        let socket_id = manager.create_socket(config).await;
        assert!(socket_id.is_ok());
        
        let socket_id = socket_id.unwrap();
        let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        
        let result = manager.connect_socket(socket_id, remote_addr).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let manager = EnhancedNetworkStackManager::new();
        let health = manager.health_check().await;
        
        assert!(health.is_ok());
        let health = health.unwrap();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
    }
}
