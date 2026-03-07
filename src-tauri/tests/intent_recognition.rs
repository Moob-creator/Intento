use intento::ai::AiClient;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// 测试用例定义
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    id: String,
    input: String,
    should_create_task: bool,  // 预期：是否应该创建任务
    expected_title_words: Option<Vec<String>>,  // 如果应该创建任务，预期的标题关键词
    category: String,  // 用例分类
}

impl TestCase {
    fn new(id: &str, input: &str, should_create_task: bool, category: &str) -> Self {
        Self {
            id: id.to_string(),
            input: input.to_string(),
            should_create_task,
            expected_title_words: None,
            category: category.to_string(),
        }
    }

    fn with_expected_keywords(mut self, keywords: Vec<&str>) -> Self {
        self.expected_title_words = Some(keywords.iter().map(|s| s.to_string()).collect());
        self
    }
}

/// 意图判断结果
#[derive(Debug, Clone, Serialize, Deserialize)]
enum IntentDecision {
    /// 应该创建任务，并提供理由
    CreateTask { reason: String },
    /// 不应该创建任务，并提供理由
    NoTask { reason: String },
}

/// 测试结果
#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    case_id: String,
    input: String,
    expected_create: bool,
    actual_create: bool,
    intent_match: bool,
    parse_accuracy: Option<f32>,  // 解析准确性评分 (0-1)
    reasoning: Option<String>,
    error: Option<String>,
}

/// 意图判断器
struct IntentAnalyzer {
    client: AiClient,
}

impl IntentAnalyzer {
    fn new(client: AiClient) -> Self {
        Self { client }
    }

    /// 判断输入是否包含创建任务的意图
    async fn analyze_intent(&self, input: &str) -> Result<IntentDecision> {
        let prompt = format!(
            r#"你是一个意图识别助手。请分析用户的输入，判断其是否包含创建待办任务的意图。

分析指南：
1. 应该创建任务的特征：
   - 明确的行动动词："要"、"需要"、"计划"、"记得"、"别忘了"、"提醒我"
   - 时间+行动："明天下午3点开会"、"下周三前完成报告"
   - 承诺表达："我承诺..."、"我会..."
   - 记录需求："记下来"、"添加到列表"、"做个备忘"

2. 不应该创建任务的特征：
   - 纯粹的询问："今天天气怎么样"、"怎么..."
   - 知识提问："解释一下...的作用"、"什么是..."
   - 娱乐请求："讲个笑话"、"唱首歌"
   - 信息查询："明天是几号"、"现在几点了"
   - 操作指令（非任务）："取消任务"、"删除记录"
   - 内容生成："写一封邮件"、"生成一段文字"
   - 反问或商讨："需要我做吗？"、"这个怎么样？"
   - 日记/笔记："今天很累"、"记录一下心情"

3. 边界情况：
   - "下午2点给客户打电话"：有时间和行动，倾向于创建任务
   - "可能需要处理"：模糊表达，可以创建任务
   - "帮忙安排会议"：需要更多信息，但可以创建任务（如"安排会议"）

请以JSON格式返回你的判断：
{{
    "should_create_task": true/false,
    "reason": "判断理由（简短，20字以内）"
}}

用户输入："{}""#,
            input
        );

        let response = self.client.chat(&prompt).await?;

        let json_str: &str = response.trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let result: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|_| anyhow::anyhow!("Failed to parse intent response as JSON: {}", response))?;

        let should_create = result["should_create_task"]
            .as_bool()
            .ok_or_else(|| anyhow::anyhow!("Missing should_create_task field"))?;

        let reason = result["reason"]
            .as_str()
            .unwrap_or("无")
            .to_string();

        if should_create {
            Ok(IntentDecision::CreateTask { reason })
        } else {
            Ok(IntentDecision::NoTask { reason })
        }
    }

    /// 解析任务
    async fn parse_task(&self, input: &str) -> Result<intento::ai::ParsedTask> {
        self.client.parse_text_input(input).await
    }
}

/// 计算解析准确性
fn calculate_parse_accuracy(
    parsed_title: &str,
    expected_keywords: &[String],
) -> f32 {
    if expected_keywords.is_empty() {
        return 1.0; // 没有预期关键词，默认准确
    }

    let mut match_count = 0;
    for keyword in expected_keywords {
        if parsed_title.contains(keyword) {
            match_count += 1;
        }
    }

    match_count as f32 / expected_keywords.len() as f32
}

/// 生成测试报告
fn generate_report(results: &[TestResult]) {
    println!("\n{:=<80}\n", "=");
    println!("测试报告 - 意图识别测试");
    println!("{:=<80}\n", "=");

    // 统计总体结果
    let total = results.len();
    let create_matches = results.iter()
        .filter(|r| r.expected_create && r.actual_create && r.intent_match)
        .count();
    let no_create_matches = results.iter()
        .filter(|r| !r.expected_create && !r.actual_create && r.intent_match)
        .count();
    let intent_correct = results.iter().filter(|r| r.intent_match).count();
    let errors = results.iter().filter(|r| r.error.is_some()).count();

    println!("总体统计：");
    println!("  总用例数: {}", total);
    println!("  意图识别正确: {} / {} ({:.1}%)",
        intent_correct, total,
        (intent_correct as f32 / total as f32) * 100.0
    );
    println!("  创建任务正确: {} / {}", create_matches,
        results.iter().filter(|r| r.expected_create).count()
    );
    println!("  不创建正确: {} / {}", no_create_matches,
        results.iter().filter(|r| !r.expected_create).count()
    );
    println!("  错误数: {}", errors);

    // 详细结果
    println!("\n详细结果：");
    println!("{:=<80}", "");
    println!("{:<6} {:<30} {:<10} {:<10} {:<8} {:<15}",
        "ID", "输入", "预期", "实际", "正确?", "理由/错误"
    );
    println!("{:=<80}", "");

    for result in results {
        let expected_str = if result.expected_create { "创建" } else { "不创" };
        let actual_str = if result.actual_create { "创建" } else { "不创" };
        let match_str = if result.intent_match { "✓" } else { "✗" };
        let reason_or_error = result.reasoning.clone()
            .unwrap_or_else(|| result.error.clone().unwrap_or_default());

        println!("{:<6} {:<30} {:<10} {:<10} {:<8} {:<15}",
            result.case_id,
            &result.input.chars().take(30).collect::<String>(),
            expected_str,
            actual_str,
            match_str,
            &reason_or_error.chars().take(15).collect::<String>()
        );
    }

    println!("{:=<80}", "");
}

#[tokio::test]
#[ignore] // 需要API密钥，默认不运行
async fn test_intent_recognition() -> Result<()> {
    // 初始化环境变量
    dotenv::dotenv().ok();

    println!("初始化AI客户端...");
    let client = AiClient::new_default()?;
    println!("✓ AI客户端初始化成功 (Provider: {:?})", client.provider());

    let analyzer = IntentAnalyzer::new(client);

    // 定义测试用例
    let test_cases = vec![
        // 基础测试用例 (图片中的)
        TestCase::new("001", "今天要买牛奶", true, "基础任务")
            .with_expected_keywords(vec!["买", "牛奶"]),
        TestCase::new("002", "今天天气怎么样", false, "询问天气"),
        TestCase::new("003", "帮我分析一下这个项目的架构", false, "知识提问"),
        TestCase::new("004", "下午3点开会，准备会议资料", true, "时间+任务")
            .with_expected_keywords(vec!["开会", "准备"]),
        TestCase::new("005", "给我讲个笑话", false, "娱乐请求"),
        TestCase::new("006", "明天是周六吗", false, "信息查询"),
        TestCase::new("007", "截止日期是下周三的报告需要完成", true, "任务+截止")
            .with_expected_keywords(vec!["报告", "完成"]),
        TestCase::new("008", "计算123+456的结果", false, "计算请求"),
        TestCase::new("009", "下午2点给客户打电话", true, "边界情况")
            .with_expected_keywords(vec!["打电话", "客户"]),

        // 补充用例 - 提醒/记录类
        TestCase::new("010", "提醒我明天晚上9点给家里打电话", true, "提醒类")
            .with_expected_keywords(vec!["提醒", "打电话"]),
        TestCase::new("011", "帮我记下来下周要提交报告", true, "记录类")
            .with_expected_keywords(vec!["记", "报告"]),
        TestCase::new("012", "帮我安排一下明天的会议", true, "安排类")
            .with_expected_keywords(vec!["安排", "会议"]),

        // 补充用例 - 模糊表达
        TestCase::new("013", "把这个想法记下来：可能需要优化代码", true, "模糊任务")
            .with_expected_keywords(vec!["优化", "代码"]),

        // 补充用例 - 明确拒绝
        TestCase::new("014", "只是问问，不用记录任务", false, "明确拒绝"),
        TestCase::new("015", "取消今天下午的任务", false, "操作指令"),
        TestCase::new("016", "不要创建这个任务", false, "明确拒绝"),

        // 补充用例 - 优先级
        TestCase::new("017", "紧急处理客户的投诉", true, "优先级")
            .with_expected_keywords(vec!["处理", "投诉"]),

        // 补充用例 - 非任务场景
        TestCase::new("018", "今天工作很辛苦", false, "日记类"),
        TestCase::new("019", "写一封给老板的邮件", false, "内容生成"),
        TestCase::new("020", "这个功能还需要实现吗？", false, "商讨类"),

        // 更多边界情况
        TestCase::new("021", "可能需要处理这个问题", true, "模糊任务")
            .with_expected_keywords(vec!["处理"]),
        TestCase::new("022", "考虑一下是否要做这个项目", false, "商讨类"),
        TestCase::new("023", "记得明天买菜", true, "提醒类")
            .with_expected_keywords(vec!["买", "菜"]),
        TestCase::new("024", "能不能帮我打印这份文件？", false, "其他请求"),
    ];

    println!("开始运行 {} 个测试用例...\n", test_cases.len());

    let mut results = Vec::new();

    for test_case in test_cases {
        print!("[{}] \"{}\" ... ", test_case.id, test_case.input);

        let result = match analyzer.analyze_intent(&test_case.input).await {
            Ok(decision) => {
                let (actual_create, reasoning) = match decision {
                    IntentDecision::CreateTask { reason } => (true, Some(reason)),
                    IntentDecision::NoTask { reason } => (false, Some(reason)),
                };

                let intent_match = actual_create == test_case.should_create_task;

                let parse_accuracy = if actual_create {
                    match analyzer.parse_task(&test_case.input).await {
                        Ok(parsed) => {
                            Some(calculate_parse_accuracy(
                                &parsed.title,
                                test_case.expected_title_words.as_deref().unwrap_or(&[])
                            ))
                        }
                        Err(e) => {
                            print!("❌ 解析错误: {} ", e);
                            None
                        }
                    }
                } else {
                    None
                };

                TestResult {
                    case_id: test_case.id.clone(),
                    input: test_case.input.clone(),
                    expected_create: test_case.should_create_task,
                    actual_create,
                    intent_match,
                    parse_accuracy,
                    reasoning: reasoning.clone(),
                    error: None,
                }
            }
            Err(e) => {
                println!("❌ 测试失败: {}", e);
                TestResult {
                    case_id: test_case.id.clone(),
                    input: test_case.input.clone(),
                    expected_create: test_case.should_create_task,
                    actual_create: false,
                    intent_match: false,
                    parse_accuracy: None,
                    reasoning: None,
                    error: Some(e.to_string()),
                }
            }
        };

        let status = if result.error.is_some() {
            "❌"
        } else if result.intent_match {
            "✓"
        } else {
            "✗"
        };

        if let Some(ref reason) = result.reasoning {
            print!("{} ({})\n", status, reason);
        } else {
            print!("{}\n", status);
        }

        results.push(result);
    }

    // 生成报告
    generate_report(&results);

    // 保存结果到JSON
    let json_output = serde_json::to_string_pretty(&results)?;
    std::fs::write("intent_test_results.json", &json_output)?;
    println!("\n结果已保存到 intent_test_results.json");

    Ok(())
}
