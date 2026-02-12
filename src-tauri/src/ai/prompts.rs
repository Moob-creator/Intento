/// Prompt template for parsing text input to structured task information
pub const PARSE_TASK_SYSTEM_PROMPT: &str = r#"You are a task parsing assistant. Your job is to extract structured task information from natural language input.

Parse the user's input and return a JSON object with the following structure:
{
    "title": "Brief task title (required)",
    "description": "Detailed description (optional)",
    "deadline": "ISO8601 datetime string (optional, e.g., '2024-12-31T23:59:59Z')",
    "priority": "low, medium, or high (optional)",
    "tags": ["tag1", "tag2"] (optional array of strings)
}

Guidelines:
1. Extract a concise title that captures the main action or goal
2. Include any additional context as description
3. Parse time references (e.g., "tomorrow", "next week", "in 3 days") into ISO8601 format
4. Infer priority from urgency indicators (e.g., "urgent", "ASAP" -> high, "when you can" -> low)
5. Extract relevant tags from context (e.g., "work meeting" -> ["work", "meeting"])
6. Only include fields that have valid information
7. Always return valid JSON, never include explanatory text

Current time: {current_time}
"#;

/// Prompt template for parsing image input to structured task information
pub const PARSE_IMAGE_SYSTEM_PROMPT: &str = r#"You are a task parsing assistant with vision capabilities. Your job is to extract structured task information from images (screenshots, photos, handwritten notes, etc.).

Analyze the image and extract any task-related information. Return a JSON object with this structure:
{
    "title": "Brief task title (required)",
    "description": "Detailed description extracted from image (optional)",
    "deadline": "ISO8601 datetime string if found (optional, e.g., '2024-12-31T23:59:59Z')",
    "priority": "low, medium, or high (optional)",
    "tags": ["tag1", "tag2"] (optional array of strings)
}

What to look for in the image:
1. Text content - meeting notes, todo lists, calendar entries, sticky notes
2. Time information - dates, deadlines, meeting times
3. Priority indicators - "urgent", "important", "ASAP", exclamation marks, red text
4. Context clues - email subjects, chat messages, document headers
5. Visual indicators - checkboxes, bullet points, numbered lists
6. Handwritten notes - recognize handwriting if present

Guidelines:
1. Extract the main task/action as the title
2. Include all relevant text from the image as description
3. Convert any date/time information to ISO8601 format
4. Infer priority from visual emphasis (bold, red, underlined, exclamation marks)
5. Extract relevant context as tags
6. If the image contains multiple tasks, focus on the most prominent one
7. If no task information is found, create a descriptive title based on image content
8. Always return valid JSON, never include explanatory text

Current time: {current_time}
"#;

/// Constructs the user prompt for task parsing
pub fn build_parse_task_prompt(user_input: &str, current_time: &str) -> String {
    format!(
        "{}\n\nUser input: {}",
        PARSE_TASK_SYSTEM_PROMPT.replace("{current_time}", current_time),
        user_input
    )
}

/// Constructs the user prompt for task parsing with existing tags context
pub fn build_parse_task_prompt_with_tags(
    user_input: &str,
    current_time: &str,
    existing_tags: Option<&[String]>,
) -> String {
    let base_prompt = PARSE_TASK_SYSTEM_PROMPT.replace("{current_time}", current_time);

    let tag_context = if let Some(tags) = existing_tags {
        if tags.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nExisting tags in the system: [{}]\n\
                IMPORTANT Tag Guidelines:\n\
                - PREFER using existing tags when they match the task context\n\
                - If the task clearly relates to an existing tag (e.g., task mentions 'work' and 'work' tag exists), use it\n\
                - If no existing tags match or you're uncertain, use the tag: \"待分类\" (pending classification)\n\
                - This allows users to manually classify tasks later\n\
                - Examples:\n\
                  * Task: \"完成工作报告\" + existing tag \"工作\" → use [\"工作\"]\n\
                  * Task: \"买菜\" + no matching tags → use [\"待分类\"]\n\
                  * Task: \"Fix bug in authentication\" + existing tag \"bug\" → use [\"bug\"]",
                tags.join(", ")
            )
        }
    } else {
        "\n\nNo existing tags. Use [\"待分类\"] for unclassified tasks.".to_string()
    };

    format!(
        "{}{}\n\nUser input: {}",
        base_prompt, tag_context, user_input
    )
}

/// Constructs the user prompt for image-based task parsing
pub fn build_parse_image_prompt(current_time: &str) -> String {
    PARSE_IMAGE_SYSTEM_PROMPT.replace("{current_time}", current_time)
}

/// Constructs the user prompt for image-based task parsing with existing tags context
pub fn build_parse_image_prompt_with_tags(
    current_time: &str,
    existing_tags: Option<&[String]>,
) -> String {
    let base_prompt = PARSE_IMAGE_SYSTEM_PROMPT.replace("{current_time}", current_time);

    let tag_context = if let Some(tags) = existing_tags {
        if tags.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nExisting tags in the system: [{}]\n\
                IMPORTANT Tag Guidelines:\n\
                - PREFER using existing tags when they match the task context\n\
                - If the image content clearly relates to an existing tag, use it\n\
                - If no existing tags match or you're uncertain, use the tag: \"待分类\" (pending classification)\n\
                - This allows users to manually classify tasks later",
                tags.join(", ")
            )
        }
    } else {
        "\n\nNo existing tags. Use [\"待分类\"] for unclassified tasks.".to_string()
    };

    format!("{}{}", base_prompt, tag_context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_parse_task_prompt() {
        let user_input = "Finish report by tomorrow";
        let current_time = "2024-01-15T10:00:00Z";
        let prompt = build_parse_task_prompt(user_input, current_time);

        assert!(prompt.contains(user_input));
        assert!(prompt.contains(current_time));
        assert!(prompt.contains("title"));
        assert!(prompt.contains("ISO8601"));
    }

    #[test]
    fn test_prompt_contains_guidelines() {
        let prompt = PARSE_TASK_SYSTEM_PROMPT;

        assert!(prompt.contains("JSON"));
        assert!(prompt.contains("priority"));
        assert!(prompt.contains("tags"));
        assert!(prompt.contains("deadline"));
    }
}
