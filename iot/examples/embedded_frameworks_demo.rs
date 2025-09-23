//! 嵌入式框架演示 - 展示RTIC、Embassy、ESP32和RISC-V集成
//! 
//! 本示例展示了如何使用最新的嵌入式框架：
//! - RTIC 1.0 实时任务管理
//! - Embassy 异步嵌入式框架
//! - ESP32 平台支持
//! - RISC-V 架构支持

//use std::time::Duration;
//use tokio::time::sleep;

// 根据特性条件编译
#[cfg(feature = "rtic")]
use iot::{
    RTICSystemManager, RTICTaskConfig, RTICTaskStatus, RTICSystemStats,
    RTICError
};

#[cfg(feature = "embassy-full")]
use iot::{
    EmbassySystemManager, EmbassyTaskConfig, EmbassyTaskStatus, EmbassySystemStats,
    EmbassyError
};

#[cfg(feature = "esp32")]
use iot::{
    ESP32Manager, ESP32Config, ESP32ChipType, ESP32SystemInfo, ESP32Error
};

#[cfg(feature = "riscv")]
use iot::{
    RISCVManager, RISCVConfig, RISCVArchType, RISCVSystemInfo, 
    RISCVPerformanceStats
};

use iot::{
    EmbeddedFrameworkManager,
    //FrameworkStatus,
};

/// RTIC实时任务演示
#[cfg(feature = "rtic")]
async fn rtic_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RTIC 1.0 实时任务管理演示 ===");
    
    let mut rtic_manager = RTICSystemManager::new();
    
    // 添加实时任务
    let task_configs = vec![
        RTICTaskConfig {
            task_id: 1,
            priority: 10,
            period_ms: 100,
            deadline_ms: 50,
            stack_size: 1024,
        },
        RTICTaskConfig {
            task_id: 2,
            priority: 8,
            period_ms: 200,
            deadline_ms: 100,
            stack_size: 512,
        },
        RTICTaskConfig {
            task_id: 3,
            priority: 6,
            period_ms: 500,
            deadline_ms: 250,
            stack_size: 256,
        },
    ];
    
    // 添加任务到RTIC管理器
    for config in task_configs {
        match rtic_manager.add_task(config) {
            Ok(task_id) => println!("成功添加任务 ID: {}", task_id),
            Err(e) => println!("添加任务失败: {}", e),
        }
    }
    
    // 启动任务
    for task_id in 1..=3 {
        if let Err(e) = rtic_manager.start_task(task_id) {
            println!("启动任务 {} 失败: {}", task_id, e);
        } else {
            println!("任务 {} 已启动", task_id);
        }
    }
    
    // 模拟任务执行
    for i in 0..10 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        let stats = rtic_manager.get_system_stats();
        println!("周期 {}: 运行任务数={}, CPU利用率={:.2}%", 
            i, stats.running_tasks, stats.cpu_utilization * 100.0);
    }
    
    // 显示最终统计
    let final_stats = rtic_manager.get_system_stats();
    println!("\nRTIC系统统计:");
    println!("  运行时间: {:?}", final_stats.uptime);
    println!("  总任务数: {}", final_stats.total_tasks);
    println!("  运行任务数: {}", final_stats.running_tasks);
    println!("  总周期数: {}", final_stats.total_cycles);
    println!("  截止时间错过: {}", final_stats.total_deadline_misses);
    println!("  CPU利用率: {:.2}%", final_stats.cpu_utilization * 100.0);
    
    Ok(())
}

/// Embassy异步任务演示
#[cfg(feature = "embassy-full")]
async fn embassy_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Embassy 异步嵌入式框架演示 ===");
    
    let mut embassy_manager = EmbassySystemManager::new();
    
    // 添加异步任务
    let task_configs = vec![
        EmbassyTaskConfig {
            name: "传感器数据采集".to_string(),
            priority: 5,
            stack_size: 2048,
            is_async: true,
        },
        EmbassyTaskConfig {
            name: "网络通信".to_string(),
            priority: 7,
            stack_size: 4096,
            is_async: true,
        },
        EmbassyTaskConfig {
            name: "数据处理".to_string(),
            priority: 3,
            stack_size: 1024,
            is_async: true,
        },
    ];
    
    // 添加任务到Embassy管理器
    for config in task_configs {
        match embassy_manager.add_async_task(config) {
            Ok(task_id) => println!("成功添加异步任务 ID: {}", task_id),
            Err(e) => println!("添加异步任务失败: {}", e),
        }
    }
    
    // 启动任务
    for task_id in 0..3 {
        if let Err(e) = embassy_manager.start_task(task_id) {
            println!("启动任务 {} 失败: {}", task_id, e);
        } else {
            println!("异步任务 {} 已启动", task_id);
        }
    }
    
    // 模拟任务执行
    for i in 0..5 {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        
        let stats = embassy_manager.get_system_stats();
        println!("周期 {}: 运行任务数={}, 待处理任务数={}, 成功率={:.2}%", 
            i, stats.running_tasks, stats.pending_tasks, stats.success_rate * 100.0);
    }
    
    // 完成任务
    for task_id in 0..3 {
        if let Err(e) = embassy_manager.complete_task(task_id) {
            println!("完成任务 {} 失败: {}", task_id, e);
        } else {
            println!("异步任务 {} 已完成", task_id);
        }
    }
    
    // 显示最终统计
    let final_stats = embassy_manager.get_system_stats();
    println!("\nEmbassy系统统计:");
    println!("  运行时间: {:?}", final_stats.uptime);
    println!("  总任务数: {}", final_stats.total_tasks);
    println!("  运行任务数: {}", final_stats.running_tasks);
    println!("  待处理任务数: {}", final_stats.pending_tasks);
    println!("  完成任务数: {}", final_stats.completed_tasks);
    println!("  失败任务数: {}", final_stats.failed_tasks);
    println!("  成功率: {:.2}%", final_stats.success_rate * 100.0);
    
    Ok(())
}

/// ESP32平台演示
#[cfg(feature = "esp32")]
async fn esp32_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ESP32 平台支持演示 ===");
    
    // 创建ESP32配置
    let config = ESP32Config {
        chip_type: ESP32ChipType::ESP32,
        cpu_frequency_mhz: 240,
        flash_size_mb: 16,
        psram_size_mb: 8,
        wifi_enabled: true,
        bluetooth_enabled: true,
    };
    
    let mut esp32_manager = ESP32Manager::new(config);
    
    // 获取系统信息
    let system_info = esp32_manager.get_system_info();
    println!("ESP32系统信息:");
    println!("  芯片类型: {:?}", system_info.config.chip_type);
    println!("  CPU频率: {} MHz", system_info.config.cpu_frequency_mhz);
    println!("  闪存大小: {} MB", system_info.config.flash_size_mb);
    println!("  PSRAM大小: {} MB", system_info.config.psram_size_mb);
    println!("  可用堆内存: {} bytes", system_info.free_heap);
    println!("  最小可用堆内存: {} bytes", system_info.min_free_heap);
    println!("  最大可分配堆内存: {} bytes", system_info.max_alloc_heap);
    println!("  运行时间: {:?}", system_info.uptime);
    println!("  重置原因: {}", system_info.reset_reason);
    println!("  CPU使用率: {:.2}%", system_info.cpu_usage * 100.0);
    
    // 初始化WiFi
    match esp32_manager.init_wifi() {
        Ok(_) => println!("WiFi初始化成功"),
        Err(e) => println!("WiFi初始化失败: {}", e),
    }
    
    // 初始化蓝牙
    match esp32_manager.init_bluetooth() {
        Ok(_) => println!("蓝牙初始化成功"),
        Err(e) => println!("蓝牙初始化失败: {}", e),
    }
    
    // 模拟深度睡眠
    match esp32_manager.enter_deep_sleep(std::time::Duration::from_secs(1)) {
        Ok(_) => println!("进入深度睡眠模式成功"),
        Err(e) => println!("进入深度睡眠模式失败: {}", e),
    }
    
    Ok(())
}

/// RISC-V架构演示
#[cfg(feature = "riscv")]
async fn riscv_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== RISC-V 架构支持演示 ===");
    
    // 创建RISC-V配置
    let config = RISCVConfig {
        arch_type: RISCVArchType::RV32IMC,
        hart_count: 1,
        memory_size_kb: 1024,
        instruction_cache_size_kb: 32,
        data_cache_size_kb: 32,
    };
    
    let mut riscv_manager = RISCVManager::new(config);
    
    // 获取系统信息
    let system_info = riscv_manager.get_system_info();
    println!("RISC-V系统信息:");
    println!("  架构类型: {:?}", system_info.config.arch_type);
    println!("  Hart数量: {}", system_info.config.hart_count);
    println!("  内存大小: {} KB", system_info.config.memory_size_kb);
    println!("  指令缓存大小: {} KB", system_info.config.instruction_cache_size_kb);
    println!("  数据缓存大小: {} KB", system_info.config.data_cache_size_kb);
    println!("  运行时间: {:?}", system_info.uptime);
    println!("  指令计数: {}", system_info.instruction_count);
    println!("  周期计数: {}", system_info.cycle_count);
    println!("  缓存命中率: {:.2}%", system_info.cache_hit_rate * 100.0);
    println!("  内存使用率: {:.2}%", system_info.memory_usage * 100.0);
    
    // 模拟指令执行
    println!("\n模拟指令执行:");
    for i in 0..1000 {
        riscv_manager.execute_instruction();
        
        if i % 100 == 0 {
            let stats = riscv_manager.get_performance_stats();
            println!("  执行 {} 条指令: IPS={:.0}, CPI={:.2}", 
                i, stats.instructions_per_second, stats.cycles_per_instruction);
        }
    }
    
    // 显示最终性能统计
    let final_stats = riscv_manager.get_performance_stats();
    println!("\nRISC-V性能统计:");
    println!("  指令执行速度: {:.0} IPS", final_stats.instructions_per_second);
    println!("  周期每指令: {:.2} CPI", final_stats.cycles_per_instruction);
    println!("  总指令数: {}", final_stats.total_instructions);
    println!("  总周期数: {}", final_stats.total_cycles);
    
    Ok(())
}

/// 统一框架管理器演示
async fn unified_framework_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 统一嵌入式框架管理器演示 ===");
    
    // 初始化各种框架
    let framework_manager = {
        #[cfg(any(feature = "rtic", feature = "embassy-full", feature = "esp32", feature = "riscv"))]
        let mut manager = EmbeddedFrameworkManager::new();
        
        #[cfg(not(any(feature = "rtic", feature = "embassy-full", feature = "esp32", feature = "riscv")))]
        let manager = EmbeddedFrameworkManager::new();
        
        #[cfg(feature = "rtic")]
        {
            manager.init_rtic();
            println!("RTIC管理器已初始化");
        }
        
        #[cfg(feature = "embassy-full")]
        {
            manager.init_embassy();
            println!("Embassy管理器已初始化");
        }
        
        #[cfg(feature = "esp32")]
        {
            let esp32_config = ESP32Config {
                chip_type: ESP32ChipType::ESP32C3,
                cpu_frequency_mhz: 160,
                flash_size_mb: 8,
                psram_size_mb: 0,
                wifi_enabled: true,
                bluetooth_enabled: false,
            };
            manager.init_esp32(esp32_config);
            println!("ESP32管理器已初始化");
        }
        
        #[cfg(feature = "riscv")]
        {
            let riscv_config = RISCVConfig {
                arch_type: RISCVArchType::RV32IMC,
                hart_count: 1,
                memory_size_kb: 512,
                instruction_cache_size_kb: 16,
                data_cache_size_kb: 16,
            };
            manager.init_riscv(riscv_config);
            println!("RISC-V管理器已初始化");
        }
        
        manager
    };
    
    // 获取框架状态
    let _status = framework_manager.get_all_framework_status();
    println!("\n框架状态:");
    #[cfg(feature = "rtic")]
    println!("  RTIC可用: {}", _status.rtic_available);
    #[cfg(feature = "embassy-full")]
    println!("  Embassy可用: {}", _status.embassy_available);
    #[cfg(feature = "esp32")]
    println!("  ESP32可用: {}", _status.esp32_available);
    #[cfg(feature = "riscv")]
    println!("  RISC-V可用: {}", _status.riscv_available);
    
    // 当没有启用任何框架特性时，显示默认状态
    #[cfg(not(any(feature = "rtic", feature = "embassy-full", feature = "esp32", feature = "riscv")))]
    println!("  没有启用任何嵌入式框架特性");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("嵌入式框架集成演示");
    println!("===================");
    
    // 运行各种框架演示
    #[cfg(feature = "rtic")]
    rtic_demo().await?;
    
    #[cfg(feature = "embassy-full")]
    embassy_demo().await?;
    
    #[cfg(feature = "esp32")]
    esp32_demo().await?;
    
    #[cfg(feature = "riscv")]
    riscv_demo().await?;
    
    // 统一框架管理器演示
    unified_framework_demo().await?;
    
    println!("\n演示完成！");
    println!("注意：要运行特定框架的演示，请使用相应的特性标志编译：");
    println!("  cargo run --example embedded_frameworks_demo --features rtic");
    println!("  cargo run --example embedded_frameworks_demo --features embassy-full");
    println!("  cargo run --example embedded_frameworks_demo --features esp32");
    println!("  cargo run --example embedded_frameworks_demo --features riscv");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_status() {
        let _status = iot::FrameworkStatus::default();
        
        #[cfg(feature = "rtic")]
        assert!(!status.rtic_available);
        
        #[cfg(feature = "embassy-full")]
        assert!(!status.embassy_available);
        
        #[cfg(feature = "esp32")]
        assert!(!status.esp32_available);
        
        #[cfg(feature = "riscv")]
        assert!(!status.riscv_available);
    }

    #[test]
    fn test_embedded_framework_manager() {
        let manager = EmbeddedFrameworkManager::new();
        let _status = manager.get_all_framework_status();
        
        // 所有框架默认都不可用
        #[cfg(feature = "rtic")]
        assert!(!_status.rtic_available);
        
        #[cfg(feature = "embassy-full")]
        assert!(!_status.embassy_available);
        
        #[cfg(feature = "esp32")]
        assert!(!_status.esp32_available);
        
        #[cfg(feature = "riscv")]
        assert!(!_status.riscv_available);
    }
}
