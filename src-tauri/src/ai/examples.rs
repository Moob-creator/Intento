// AI 客户端使用示例
//
// 此文件演示如何在应用中使用 AI 模块
// 注意：这是示例代码，不会被编译到最终程序中

#[allow(dead_code)]
mod examples {
    use crate::ai::{AiClient, ModelProvider, ParsedTask};
    use anyhow::Result;

    /// 示例 1: 基本使用 - OpenAI
    #[allow(dead_code)]
    async fn example_basic_openai() -> Result<()> {
        // 创建 OpenAI 客户端（需要设置 OPENAI_API_KEY 环境变量）
        let client = AiClient::new(ModelProvider::OpenAI)?;

        // 解析用户输入
        let user_input = "明天下午3点开会讨论 Q4 规划，高优先级";
        let parsed = client.parse_text_input(user_input).await?;

        println!("解析结果:");
        println!("  标题: {}", parsed.title);
        if let Some(desc) = parsed.description {
            println!("  描述: {}", desc);
        }
        if let Some(deadline) = parsed.deadline {
            println!("  截止时间: {}", deadline);
        }
        if let Some(priority) = parsed.priority {
            println!("  优先级: {}", priority);
        }
        if let Some(tags) = parsed.tags {
            println!("  标签: {:?}", tags);
        }

        Ok(())
    }

    /// 示例 2: 使用 Claude
    #[allow(dead_code)]
    async fn example_claude() -> Result<()> {
        // 创建 Anthropic 客户端（需要设置 ANTHROPIC_API_KEY 环境变量）
        let client = AiClient::new(ModelProvider::Anthropic)?;

        let parsed = client
            .parse_text_input("紧急：修复生产环境 bug，ASAP")
            .await?;

        println!("任务: {}", parsed.title);
        println!("优先级: {:?}", parsed.priority); // 应该是 "high"

        Ok(())
    }

    /// 示例 3: 使用默认 provider
    #[allow(dead_code)]
    async fn example_default_provider() -> Result<()> {
        // 根据 AI_PROVIDER 环境变量选择 provider
        // 未设置时默认使用 OpenAI
        let client = AiClient::new_default()?;

        let parsed = client
            .parse_text_input("下周一提交代码审查")
            .await?;

        println!("任务: {}", parsed.title);
        Ok(())
    }

    /// 示例 4: 健康检查
    #[allow(dead_code)]
    async fn example_health_check() -> Result<()> {
        let client = AiClient::new_default()?;

        // 检查 AI 服务是否可用
        if client.health_check().await {
            println!("AI 服务正常");
        } else {
            println!("AI 服务不可用");
        }

        Ok(())
    }

    /// 示例 5: 错误处理
    #[allow(dead_code)]
    async fn example_error_handling() -> Result<()> {
        match AiClient::new_default() {
            Ok(client) => {
                match client.parse_text_input("").await {
                    Ok(parsed) => {
                        println!("解析成功: {}", parsed.title);
                    }
                    Err(e) => {
                        eprintln!("解析失败: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("初始化失败: {}", e);
                eprintln!("请检查是否设置了 OPENAI_API_KEY 或 ANTHROPIC_API_KEY");
            }
        }

        Ok(())
    }

    /// 示例 6: 批量处理
    #[allow(dead_code)]
    async fn example_batch_processing() -> Result<()> {
        let client = AiClient::new_default()?;

        let inputs = vec![
            "明天上午开会",
            "本周五提交报告，高优先级",
            "下个月计划项目评审",
        ];

        let mut results = Vec::new();
        for input in inputs {
            match client.parse_text_input(input).await {
                Ok(parsed) => {
                    println!("✓ {}: {}", input, parsed.title);
                    results.push(parsed);
                }
                Err(e) => {
                    eprintln!("✗ {}: {}", input, e);
                }
            }
        }

        println!("\n成功解析 {} 个任务", results.len());

        Ok(())
    }

    /// 示例 7: 与数据库集成
    #[allow(dead_code)]
    async fn example_with_database(
        client: &AiClient,
        db: &crate::db::Database,
    ) -> Result<()> {
        // 解析用户输入
        let user_input = "明天下午发布新版本，高优先级";
        let parsed = client.parse_text_input(user_input).await?;

        // 转换为 Task 模型
        let mut task = crate::db::models::Task::new(parsed.title.clone());

        // 设置可选字段
        if let Some(desc) = parsed.description {
            task.description = Some(desc);
        }

        if let Some(priority_str) = parsed.priority {
            task.priority = crate::db::models::Priority::from_str(&priority_str)?;
        }

        if let Some(deadline_str) = parsed.deadline {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&deadline_str) {
                task.deadline = Some(dt.timestamp());
            }
        }

        task.tags = parsed.tags;

        // 保存到数据库
        let task_id = db.create_task(&task)?;
        println!("任务已创建，ID: {}", task_id);

        Ok(())
    }

    /// 示例 8: 验证解析结果
    #[allow(dead_code)]
    async fn example_validation() -> Result<()> {
        let client = AiClient::new_default()?;

        let parsed = client
            .parse_text_input("明天 18:00 完成代码审查")
            .await?;

        // 验证优先级
        if let Some(ref priority) = parsed.priority {
            match priority.as_str() {
                "low" | "medium" | "high" => {
                    println!("优先级有效: {}", priority);
                }
                _ => {
                    println!("优先级无效: {}", priority);
                }
            }
        }

        // 验证截止时间
        if let Some(ref deadline_str) = parsed.deadline {
            match chrono::DateTime::parse_from_rfc3339(deadline_str) {
                Ok(dt) => {
                    println!("截止时间: {}", dt.format("%Y-%m-%d %H:%M"));
                }
                Err(e) => {
                    println!("截止时间格式错误: {}", e);
                }
            }
        }

        Ok(())
    }
}

// Tauri Command 使用示例（前端调用）
/*
TypeScript/JavaScript:

import { invoke } from '@tauri-apps/api/core';

// 解析任务
const parsed = await invoke('parse_text_input', {
  text: '明天下午开会，重要'
});

console.log(parsed);
// {
//   title: "开会",
//   description: null,
//   deadline: "2024-12-01T14:00:00Z",
//   priority: "high",
//   tags: ["meeting"]
// }

// 检查健康状态
const healthy = await invoke('ai_health_check');
if (!healthy) {
  alert('AI 服务暂时不可用');
}

// 获取 provider
const provider = await invoke('get_ai_provider');
console.log(`当前使用: ${provider}`);
*/
