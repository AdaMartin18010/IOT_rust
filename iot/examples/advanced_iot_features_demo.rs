//! 高级IoT功能演示 - 展示最新技术集成
//! 
//! 本示例展示了最新的IoT技术：
//! - 数字孪生技术
//! - 边缘AI推理
//! - 量子加密通信
//! - 5G网络切片
//! - 实时数据分析

use std::time::{Duration, Instant};
use tokio::time::sleep;
use iot::{
    DigitalTwinManager, TwinType, TwinProperty, PropertyValue, PropertyDataType,
    EdgeAIEngine, AIModel, ModelType, InferenceTask, TaskPriority,
    QuantumCryptoManager, SecurityLevel, QKDProtocol, QKDProtocolType,
    Network5GSliceManager, SliceType, SliceConfig, SliceResources, SLARequirements,
};

/// 数字孪生演示
async fn digital_twin_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 数字孪生技术演示 ===");
    
    let mut twin_manager = DigitalTwinManager::<100>::new(Duration::from_secs(5));
    
    // 创建智能工厂的数字孪生
    let factory_twins = vec![
        ("factory_001", "智能工厂A", TwinType::PhysicalDevice),
        ("production_line_001", "生产线1", TwinType::SystemComponent),
        ("robot_001", "工业机器人1", TwinType::PhysicalDevice),
        ("sensor_001", "温度传感器1", TwinType::PhysicalDevice),
        ("quality_control", "质量控制系统", TwinType::Process),
    ];
    
    for (id, name, twin_type) in factory_twins {
        match twin_manager.create_twin(id.to_string(), name.to_string(), twin_type) {
            Ok(_) => println!("✓ 创建数字孪生: {} ({})", name, id),
            Err(e) => println!("✗ 创建数字孪生失败: {}", e),
        }
    }
    
    // 模拟传感器数据更新
    let sensor_data = vec![
        ("sensor_001", "temperature", PropertyValue::Float(25.5)),
        ("sensor_001", "humidity", PropertyValue::Float(60.2)),
        ("robot_001", "position_x", PropertyValue::Float(100.0)),
        ("robot_001", "position_y", PropertyValue::Float(200.0)),
        ("robot_001", "status", PropertyValue::String("运行中".to_string())),
        ("production_line_001", "throughput", PropertyValue::Integer(150)),
        ("production_line_001", "efficiency", PropertyValue::Float(95.5)),
    ];
    
    for (twin_id, property_name, value) in sensor_data {
        let property = TwinProperty {
            name: property_name.to_string(),
            value,
            data_type: match property_name {
                "temperature" => PropertyDataType::Temperature,
                "humidity" => PropertyDataType::Humidity,
                "position_x" | "position_y" => PropertyDataType::Custom("坐标".to_string()),
                "status" => PropertyDataType::Status,
                "throughput" => PropertyDataType::Custom("产量".to_string()),
                "efficiency" => PropertyDataType::Custom("效率".to_string()),
                _ => PropertyDataType::Custom("自定义".to_string()),
            },
            unit: match property_name {
                "temperature" => Some("°C".to_string()),
                "humidity" => Some("%".to_string()),
                "position_x" | "position_y" => Some("mm".to_string()),
                "efficiency" => Some("%".to_string()),
                _ => None,
            },
            timestamp: Instant::now().into(),
            quality: 0.95,
        };
        
        if let Err(e) = twin_manager.update_property(twin_id, property) {
            println!("✗ 更新属性失败: {}", e);
        } else {
            println!("✓ 更新属性: {} -> {}", twin_id, property_name);
        }
    }
    
    // 同步数字孪生
    match twin_manager.sync_twins() {
        Ok(sync_result) => {
            println!("✓ 数字孪生同步完成:");
            println!("  同步数量: {}", sync_result.synced_count);
            println!("  错误数量: {}", sync_result.error_count);
            println!("  同步耗时: {:?}", sync_result.sync_duration);
        }
        Err(e) => println!("✗ 数字孪生同步失败: {}", e),
    }
    
    // 显示统计信息
    let stats = twin_manager.get_twin_stats();
    println!("\n数字孪生统计:");
    println!("  总孪生数: {}", stats.total_twins);
    println!("  活跃孪生数: {}", stats.active_twins);
    println!("  非活跃孪生数: {}", stats.inactive_twins);
    println!("  错误孪生数: {}", stats.error_twins);
    println!("  最后同步时间: {:?}", stats.last_sync);
    
    Ok(())
}

/// 边缘AI推理演示
async fn edge_ai_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 边缘AI推理演示 ===");
    
    let mut ai_engine = EdgeAIEngine::new();
    
    // 添加AI模型
    let models = vec![
        AIModel {
            id: "anomaly_detection".to_string(),
            name: "异常检测模型".to_string(),
            model_type: ModelType::AnomalyDetection,
            version: "1.0".to_string(),
            input_size: 10,
            output_size: 1,
            accuracy: 0.95,
            latency_ms: 25.0,
            memory_usage_mb: 50.0,
        },
        AIModel {
            id: "quality_classification".to_string(),
            name: "质量分类模型".to_string(),
            model_type: ModelType::Classification,
            version: "2.1".to_string(),
            input_size: 20,
            output_size: 4,
            accuracy: 0.98,
            latency_ms: 40.0,
            memory_usage_mb: 80.0,
        },
        AIModel {
            id: "predictive_maintenance".to_string(),
            name: "预测性维护模型".to_string(),
            model_type: ModelType::TimeSeriesForecasting,
            version: "1.5".to_string(),
            input_size: 15,
            output_size: 3,
            accuracy: 0.92,
            latency_ms: 60.0,
            memory_usage_mb: 120.0,
        },
    ];
    
    for model in models {
        match ai_engine.add_model(model.clone()) {
            Ok(_) => println!("✓ 添加AI模型: {}", model.name),
            Err(e) => println!("✗ 添加AI模型失败: {}", e),
        }
    }
    
    // 提交推理任务
    let inference_tasks = vec![
        InferenceTask {
            id: "task_001".to_string(),
            model_id: "anomaly_detection".to_string(),
            input_data: vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9, 9.0, 10.1],
            priority: TaskPriority::High,
            created_at: Instant::now().into(),
            deadline: Some((Instant::now() + Duration::from_millis(100)).into()),
        },
        InferenceTask {
            id: "task_002".to_string(),
            model_id: "quality_classification".to_string(),
            input_data: vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0],
            priority: TaskPriority::Normal,
            created_at: Instant::now().into(),
            deadline: None,
        },
        InferenceTask {
            id: "task_003".to_string(),
            model_id: "predictive_maintenance".to_string(),
            input_data: vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0],
            priority: TaskPriority::Critical,
            created_at: Instant::now().into(),
            deadline: Some((Instant::now() + Duration::from_millis(200)).into()),
        },
    ];
    
    for task in inference_tasks {
        match ai_engine.submit_inference(task.clone()) {
            Ok(_) => println!("✓ 提交推理任务: {}", task.id),
            Err(e) => println!("✗ 提交推理任务失败: {}", e),
        }
    }
    
    // 处理推理任务
    println!("\n处理推理任务:");
    for _i in 0..5 {
        if let Some(result) = ai_engine.process_inference() {
            println!("  任务 {}: 模型={}, 成功={}, 置信度={:.2}, 耗时={:?}", 
                result.task_id, result.model_id, result.success, 
                result.confidence, result.processing_time);
        } else {
            println!("  队列为空，等待新任务...");
        }
        sleep(Duration::from_millis(50)).await;
    }
    
    // 显示AI引擎统计
    let stats = ai_engine.get_stats();
    println!("\nAI引擎统计:");
    println!("  总推理次数: {}", stats.total_inferences);
    println!("  成功推理次数: {}", stats.successful_inferences);
    println!("  失败推理次数: {}", stats.failed_inferences);
    println!("  平均延迟: {:.2} ms", stats.average_latency_ms);
    println!("  队列大小: {}", stats.queue_size);
    
    Ok(())
}

/// 量子加密通信演示
async fn quantum_crypto_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 量子加密通信演示 ===");
    
    let mut quantum_manager = QuantumCryptoManager::new(SecurityLevel::High);
    
    // 添加QKD协议
    let protocols = vec![
        QKDProtocol {
            id: "bb84_protocol".to_string(),
            protocol_type: QKDProtocolType::BB84,
            key_rate_bps: 1000.0,
            error_rate: 0.01,
            distance_km: 50.0,
            security_parameter: 128,
        },
        QKDProtocol {
            id: "e91_protocol".to_string(),
            protocol_type: QKDProtocolType::E91,
            key_rate_bps: 800.0,
            error_rate: 0.005,
            distance_km: 100.0,
            security_parameter: 256,
        },
        QKDProtocol {
            id: "mdi_protocol".to_string(),
            protocol_type: QKDProtocolType::MeasurementDeviceIndependent,
            key_rate_bps: 500.0,
            error_rate: 0.02,
            distance_km: 200.0,
            security_parameter: 512,
        },
    ];
    
    for protocol in protocols {
        quantum_manager.add_qkd_protocol(protocol.clone());
        println!("✓ 添加QKD协议: {:?}", protocol.protocol_type);
    }
    
    // 生成量子密钥
    let key_lengths = vec![128, 256, 512];
    let mut generated_keys = Vec::new();
    
    for length in key_lengths {
        match quantum_manager.generate_quantum_key(length) {
            Ok(key) => {
                println!("✓ 生成量子密钥: ID={}, 长度={} bits", key.id, key.key_length * 8);
                generated_keys.push(key);
            }
            Err(e) => println!("✗ 生成量子密钥失败: {}", e),
        }
    }
    
    // 测试加密解密
    if let Some(key) = generated_keys.first() {
        let test_data = "这是需要量子加密保护的重要IoT数据！".as_bytes();
        println!("\n测试数据: {}", String::from_utf8_lossy(test_data));
        
        match quantum_manager.encrypt_data(test_data, &key.id) {
            Ok(encrypted) => {
                println!("✓ 数据加密成功，加密后长度: {} bytes", encrypted.len());
                
                match quantum_manager.decrypt_data(&encrypted, &key.id) {
                    Ok(decrypted) => {
                        let decrypted_str = String::from_utf8_lossy(&decrypted);
                        println!("✓ 数据解密成功: {}", decrypted_str);
                        
                        if test_data == decrypted.as_slice() {
                            println!("✓ 加密解密验证成功！");
                        } else {
                            println!("✗ 加密解密验证失败！");
                        }
                    }
                    Err(e) => println!("✗ 数据解密失败: {}", e),
                }
            }
            Err(e) => println!("✗ 数据加密失败: {}", e),
        }
    }
    
    // 显示量子安全统计
    let stats = quantum_manager.get_security_stats();
    println!("\n量子安全统计:");
    println!("  总协议数: {}", stats.total_protocols);
    println!("  活跃密钥数: {}", stats.active_keys);
    println!("  过期密钥数: {}", stats.expired_keys);
    println!("  密钥生成速率: {:.0} keys/sec", stats.key_generation_rate);
    println!("  安全级别: {:?}", stats.security_level);
    
    Ok(())
}

/// 5G网络切片演示
async fn network_5g_slice_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 5G网络切片演示 ===");
    
    let mut slice_manager = Network5GSliceManager::new();
    
    // 定义不同类型的网络切片配置
    let slice_configs = vec![
        (SliceType::EnhancedMobileBroadband, "eMBB切片", SliceResources {
            bandwidth_mbps: 1000.0,
            latency_ms: 10.0,
            reliability_percent: 99.0,
            compute_cores: 4,
            memory_gb: 8.0,
            storage_gb: 50.0,
        }),
        (SliceType::UltraReliableLowLatency, "URLLC切片", SliceResources {
            bandwidth_mbps: 100.0,
            latency_ms: 1.0,
            reliability_percent: 99.9,
            compute_cores: 2,
            memory_gb: 4.0,
            storage_gb: 20.0,
        }),
        (SliceType::MassiveMachineType, "mMTC切片", SliceResources {
            bandwidth_mbps: 10.0,
            latency_ms: 100.0,
            reliability_percent: 95.0,
            compute_cores: 1,
            memory_gb: 2.0,
            storage_gb: 10.0,
        }),
        (SliceType::IndustrialIoT, "工业IoT切片", SliceResources {
            bandwidth_mbps: 500.0,
            latency_ms: 5.0,
            reliability_percent: 99.5,
            compute_cores: 3,
            memory_gb: 6.0,
            storage_gb: 30.0,
        }),
    ];
    
    // 创建网络切片
    for (slice_type, name, resources) in slice_configs {
        let config = SliceConfig {
            slice_type: slice_type.clone(),
            min_resources: resources.clone(),
            max_resources: SliceResources {
                bandwidth_mbps: resources.bandwidth_mbps * 2.0,
                latency_ms: resources.latency_ms * 0.5,
                reliability_percent: 99.99,
                compute_cores: resources.compute_cores * 2,
                memory_gb: resources.memory_gb * 2.0,
                storage_gb: resources.storage_gb * 2.0,
            },
            sla_requirements: SLARequirements {
                max_latency_ms: resources.latency_ms,
                min_reliability_percent: resources.reliability_percent,
                min_throughput_mbps: resources.bandwidth_mbps * 0.8,
                max_packet_loss_rate: 0.001,
            },
        };
        
        let slice_id = format!("slice_{:?}", slice_type).to_lowercase();
        match slice_manager.create_slice(slice_id, name.to_string(), slice_type, &config) {
            Ok(_) => println!("✓ 创建网络切片: {}", name),
            Err(e) => println!("✗ 创建网络切片失败: {}", e),
        }
    }
    
    // 模拟切片性能更新
    let performance_updates = vec![
        ("slice_enhancedmobilebroadband", 950.0, 8.5, 0.0001, 99.2, 45.0, 60.0),
        ("slice_ultrareliablelowlatency", 95.0, 0.8, 0.00005, 99.95, 30.0, 40.0),
        ("slice_massivemachinetype", 9.5, 95.0, 0.001, 96.0, 20.0, 25.0),
        ("slice_industrialiot", 480.0, 4.2, 0.0002, 99.6, 35.0, 50.0),
    ];
    
    for (slice_id, throughput, latency, packet_loss, availability, cpu_usage, memory_usage) in performance_updates {
        let performance = iot::SlicePerformance {
            throughput_mbps: throughput,
            actual_latency_ms: latency,
            packet_loss_rate: packet_loss,
            availability_percent: availability,
            cpu_usage_percent: cpu_usage,
            memory_usage_percent: memory_usage,
        };
        
        if let Err(e) = slice_manager.update_slice_performance(slice_id, performance) {
            println!("✗ 更新切片性能失败: {}", e);
        } else {
            println!("✓ 更新切片性能: {} (吞吐量: {:.1} Mbps, 延迟: {:.1} ms)", 
                slice_id, throughput, latency);
        }
    }
    
    // 显示切片管理统计
    let stats = slice_manager.get_slice_stats();
    println!("\n5G网络切片统计:");
    println!("  总切片数: {}", stats.total_slices);
    println!("  活跃切片数: {}", stats.active_slices);
    println!("  配置中切片数: {}", stats.provisioning_slices);
    println!("  错误切片数: {}", stats.error_slices);
    println!("  资源利用率:");
    println!("    带宽利用率: {:.1}%", stats.resource_utilization.bandwidth_utilization * 100.0);
    println!("    计算利用率: {:.1}%", stats.resource_utilization.compute_utilization * 100.0);
    println!("    内存利用率: {:.1}%", stats.resource_utilization.memory_utilization * 100.0);
    
    Ok(())
}

/// 综合演示
async fn integrated_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 综合IoT技术演示 ===");
    
    // 创建数字孪生管理器
    let mut twin_manager = DigitalTwinManager::<50>::new(Duration::from_secs(2));
    
    // 创建智能城市数字孪生
    let city_twins = vec![
        ("smart_city", "智慧城市", TwinType::SystemComponent),
        ("traffic_management", "交通管理系统", TwinType::Process),
        ("air_quality_monitor", "空气质量监测站", TwinType::PhysicalDevice),
        ("smart_grid", "智能电网", TwinType::SystemComponent),
        ("emergency_response", "应急响应系统", TwinType::Process),
    ];
    
    for (id, name, twin_type) in city_twins {
        if let Err(e) = twin_manager.create_twin(id.to_string(), name.to_string(), twin_type) {
            println!("创建数字孪生失败: {}", e);
        }
    }
    
    // 创建AI引擎
    let mut ai_engine = EdgeAIEngine::new();
    
    // 添加城市管理AI模型
    let city_model = AIModel {
        id: "city_optimization".to_string(),
        name: "城市优化模型".to_string(),
        model_type: ModelType::TimeSeriesForecasting,
        version: "1.0".to_string(),
        input_size: 12,
        output_size: 6,
        accuracy: 0.94,
        latency_ms: 35.0,
        memory_usage_mb: 90.0,
    };
    
    if let Err(e) = ai_engine.add_model(city_model) {
        println!("添加AI模型失败: {}", e);
    }
    
    // 创建量子加密管理器
    let mut quantum_manager = QuantumCryptoManager::new(SecurityLevel::Ultra);
    
    // 生成城市通信密钥
    if let Err(e) = quantum_manager.generate_quantum_key(512) {
        println!("生成量子密钥失败: {}", e);
    }
    
    // 创建5G网络切片管理器
    let mut slice_manager = Network5GSliceManager::new();
    
    // 创建城市专用切片
    let city_slice_config = SliceConfig {
        slice_type: SliceType::IndustrialIoT,
        min_resources: SliceResources {
            bandwidth_mbps: 2000.0,
            latency_ms: 2.0,
            reliability_percent: 99.9,
            compute_cores: 8,
            memory_gb: 16.0,
            storage_gb: 100.0,
        },
        max_resources: SliceResources {
            bandwidth_mbps: 5000.0,
            latency_ms: 1.0,
            reliability_percent: 99.99,
            compute_cores: 16,
            memory_gb: 32.0,
            storage_gb: 200.0,
        },
        sla_requirements: SLARequirements {
            max_latency_ms: 2.0,
            min_reliability_percent: 99.9,
            min_throughput_mbps: 2000.0,
            max_packet_loss_rate: 0.0001,
        },
    };
    
    if let Err(e) = slice_manager.create_slice("smart_city_slice".to_string(), "智慧城市切片".to_string(), SliceType::IndustrialIoT, &city_slice_config) {
        println!("创建网络切片失败: {}", e);
    }
    
    // 模拟综合运行
    println!("\n模拟智慧城市运行:");
    for i in 0..5 {
        println!("\n--- 运行周期 {} ---", i + 1);
        
        // 更新数字孪生数据
        let city_data = vec![
            ("air_quality_monitor", "pm2_5", PropertyValue::Float(35.0 + i as f32)),
            ("air_quality_monitor", "pm10", PropertyValue::Float(50.0 + i as f32)),
            ("traffic_management", "vehicle_count", PropertyValue::Integer(1000 + i * 100)),
            ("smart_grid", "power_consumption", PropertyValue::Float(5000.0 + i as f32 * 100.0)),
        ];
        
        for (twin_id, property_name, value) in city_data {
            let property = TwinProperty {
                name: property_name.to_string(),
                value,
                data_type: PropertyDataType::Custom("监测数据".to_string()),
                unit: match property_name {
                    "pm2_5" | "pm10" => Some("μg/m³".to_string()),
                    "power_consumption" => Some("kW".to_string()),
                    _ => None,
                },
                timestamp: Instant::now().into(),
                quality: 0.98,
            };
            
            if let Err(e) = twin_manager.update_property(twin_id, property) {
                println!("更新属性失败: {}", e);
            }
        }
        
        // 提交AI推理任务
        let ai_task = InferenceTask {
            id: format!("city_task_{}", i),
            model_id: "city_optimization".to_string(),
            input_data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            priority: TaskPriority::High,
            created_at: Instant::now().into(),
            deadline: Some((Instant::now() + Duration::from_millis(100)).into()),
        };
        
        if let Err(e) = ai_engine.submit_inference(ai_task) {
            println!("提交AI任务失败: {}", e);
        }
        
        // 处理AI推理
        if let Some(result) = ai_engine.process_inference() {
            println!("AI推理结果: 任务={}, 成功={}, 置信度={:.2}", 
                result.task_id, result.success, result.confidence);
        }
        
        // 同步数字孪生
        if let Ok(sync_result) = twin_manager.sync_twins() {
            println!("数字孪生同步: 成功={}, 错误={}", 
                sync_result.synced_count, sync_result.error_count);
        }
        
        sleep(Duration::from_millis(200)).await;
    }
    
    // 显示最终统计
    println!("\n=== 综合系统统计 ===");
    
    let twin_stats = twin_manager.get_twin_stats();
    println!("数字孪生: 总数={}, 活跃={}", twin_stats.total_twins, twin_stats.active_twins);
    
    let ai_stats = ai_engine.get_stats();
    println!("AI引擎: 总推理={}, 成功率={:.1}%", 
        ai_stats.total_inferences, 
        (ai_stats.successful_inferences as f32 / ai_stats.total_inferences as f32) * 100.0);
    
    let quantum_stats = quantum_manager.get_security_stats();
    println!("量子加密: 活跃密钥={}, 安全级别={:?}", 
        quantum_stats.active_keys, quantum_stats.security_level);
    
    let slice_stats = slice_manager.get_slice_stats();
    println!("5G切片: 总切片={}, 活跃切片={}", 
        slice_stats.total_slices, slice_stats.active_slices);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("高级IoT功能演示");
    println!("================");
    println!("本演示展示了最新的IoT技术集成：");
    println!("- 数字孪生技术");
    println!("- 边缘AI推理");
    println!("- 量子加密通信");
    println!("- 5G网络切片");
    println!("- 综合系统演示");
    println!();
    
    // 运行各个演示
    digital_twin_demo().await?;
    edge_ai_demo().await?;
    quantum_crypto_demo().await?;
    network_5g_slice_demo().await?;
    integrated_demo().await?;
    
    println!("\n演示完成！");
    println!("这些技术展示了IoT领域的最新发展方向，");
    println!("结合Rust 1.90的新特性，为构建下一代智能IoT系统提供了强大的基础。");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_creation() {
        let mut manager = DigitalTwinManager::<10>::new(Duration::from_secs(1));
        
        assert!(manager.create_twin("test_twin".to_string(), "Test Twin".to_string(), TwinType::PhysicalDevice).is_ok());
        assert!(manager.create_twin("test_twin".to_string(), "Duplicate".to_string(), TwinType::PhysicalDevice).is_err());
    }

    #[test]
    fn test_ai_model_management() {
        let mut engine = EdgeAIEngine::new();
        
        let model = AIModel {
            id: "test_model".to_string(),
            name: "Test Model".to_string(),
            model_type: ModelType::Classification,
            version: "1.0".to_string(),
            input_size: 5,
            output_size: 2,
            accuracy: 0.9,
            latency_ms: 30.0,
            memory_usage_mb: 50.0,
        };
        
        assert!(engine.add_model(model).is_ok());
    }

    #[test]
    fn test_quantum_key_generation() {
        let mut manager = QuantumCryptoManager::new(SecurityLevel::Standard);
        
        assert!(manager.generate_quantum_key(128).is_ok());
        assert!(manager.generate_quantum_key(256).is_ok());
    }

    #[test]
    fn test_network_slice_creation() {
        let mut manager = Network5GSliceManager::new();
        
        let config = SliceConfig {
            slice_type: SliceType::EnhancedMobileBroadband,
            min_resources: SliceResources {
                bandwidth_mbps: 100.0,
                latency_ms: 10.0,
                reliability_percent: 99.0,
                compute_cores: 1,
                memory_gb: 2.0,
                storage_gb: 10.0,
            },
            max_resources: SliceResources {
                bandwidth_mbps: 1000.0,
                latency_ms: 1.0,
                reliability_percent: 99.9,
                compute_cores: 4,
                memory_gb: 8.0,
                storage_gb: 50.0,
            },
            sla_requirements: SLARequirements {
                max_latency_ms: 10.0,
                min_reliability_percent: 99.0,
                min_throughput_mbps: 100.0,
                max_packet_loss_rate: 0.001,
            },
        };
        
        assert!(manager.create_slice("test_slice".to_string(), "Test Slice".to_string(), SliceType::EnhancedMobileBroadband, &config).is_ok());
    }
}
