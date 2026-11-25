use std::fmt;

use rig::completion::Usage;

#[derive(Debug, Clone, Default)]
pub struct MyUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
    pub cached_input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub tool_use_prompt_tokens: Option<u64>,
    pub reasoning_tokens: Option<u64>,
}

impl fmt::Display for MyUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "input_tokens: {}, output_tokens: {}, total_tokens: {}, cached_input_tokens: {}, cache_creation_input_tokens: {}, tool_use_prompt_tokens: {}, reasoning_tokens: {}",
            self.input_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.output_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.total_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.cached_input_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.cache_creation_input_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.tool_use_prompt_tokens
                .map_or("none".to_string(), |v| v.to_string()),
            self.reasoning_tokens
                .map_or("none".to_string(), |v| v.to_string())
        )
    }
}

// This implementation assumes Usage is defined elsewhere in the codebase
// If Usage is not available at compile time, this will need to be adjusted
impl From<&Usage> for MyUsage {
    fn from(usage: &Usage) -> Self {
        MyUsage {
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            total_tokens: usage.total_tokens,
            cached_input_tokens: usage.cached_input_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            tool_use_prompt_tokens: usage.tool_use_prompt_tokens,
            reasoning_tokens: usage.reasoning_tokens,
        }
    }
}
