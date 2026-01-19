//! A Windows implementation of ExecuteCmd that uses PowerShell as the shell.
#![cfg(target_family = "windows")]

use std::collections::HashMap;
use std::process::Stdio;

use bstr::ByteSlice as _;
use schemars::{
    JsonSchema,
    schema_for,
};
use serde::{
    Deserialize,
    Serialize,
};
use tokio::process::Command;

use super::{
    BuiltInToolName,
    BuiltInToolTrait,
    ToolExecutionError,
    ToolExecutionOutput,
    ToolExecutionOutputItem,
    ToolExecutionResult,
};
use crate::agent::util::consts::{
    USER_AGENT_APP_NAME,
    USER_AGENT_ENV_VAR,
    USER_AGENT_VERSION_KEY,
    USER_AGENT_VERSION_VALUE,
};

const EXECUTE_CMD_TOOL_DESCRIPTION: &str = r#"
A tool for executing PowerShell commands.

WHEN TO USE THIS TOOL:
- Use only as a last-resort when no other available tool can accomplish the task

HOW TO USE:
- Provide the command to execute

FEATURES:

LIMITATIONS:
- Does not respect user's PowerShell profile

TIPS:
- Use the fileRead and fileWrite tools for reading and modifying files
"#;

const EXECUTE_CMD_SCHEMA: &str = r#"
{
    "type": "object",
    "properties": {
        "command": {
            "type": "string",
            "description": "Command to execute"
        }
    },
    "required": [
        "command"
    ]
}
"#;

impl BuiltInToolTrait for ExecuteCmd {
    fn name() -> BuiltInToolName {
        BuiltInToolName::ExecuteCmd
    }

    fn description() -> std::borrow::Cow<'static, str> {
        EXECUTE_CMD_TOOL_DESCRIPTION.into()
    }

    fn input_schema() -> std::borrow::Cow<'static, str> {
        EXECUTE_CMD_SCHEMA.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteCmd {
    pub command: String,
}

impl ExecuteCmd {
    pub fn tool_schema() -> serde_json::Value {
        let schema = schema_for!(Self);
        serde_json::to_value(schema).expect("creating tool schema should not fail")
    }

    pub async fn validate(&self) -> Result<(), String> {
        if self.command.is_empty() {
            Err("Command must not be empty".to_string())
        } else {
            Ok(())
        }
    }

    pub async fn execute(&self) -> ToolExecutionResult {
        let shell = std::env::var("AMAZON_Q_CHAT_SHELL").unwrap_or("pwsh".to_string());

        let mut env_vars = HashMap::new();
        env_vars.insert(USER_AGENT_ENV_VAR.to_string(), USER_AGENT_APP_NAME.to_string());
        env_vars.insert(USER_AGENT_VERSION_KEY.to_string(), USER_AGENT_VERSION_VALUE.to_string());
        crate::agent::util::expand_env_vars(&mut env_vars);

        let mut cmd = Command::new(&shell);
        cmd.arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-Command")
            .arg(&self.command)
            .envs(env_vars)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .map_err(|e| ToolExecutionError::Custom(format!("failed to execute command: {}", e)))?;

        let stdout = output.stdout.to_str_lossy().to_string();
        let stderr = output.stderr.to_str_lossy().to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push_str("\n");
            }
            result.push_str(&stderr);
        }
        if result.is_empty() {
            result = format!("Command exited with code {}", exit_code);
        }

        Ok(ToolExecutionOutput::new(vec![ToolExecutionOutputItem::Text(result)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_simple_command() {
        let tool = ExecuteCmd {
            command: "echo 'hello world'".to_string(),
        };

        assert!(tool.validate().await.is_ok());
        let result = tool.execute().await.unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn test_validate_empty_command() {
        let tool = ExecuteCmd {
            command: String::new(),
        };

        assert!(tool.validate().await.is_err());
    }

    #[tokio::test]
    async fn test_execute_with_exit_code() {
        let tool = ExecuteCmd {
            command: "exit 42".to_string(),
        };

        let result = tool.execute().await.unwrap();
        assert_eq!(result.items.len(), 1);
    }
}
