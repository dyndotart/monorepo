use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    /// HTTP status code associated with the error, indicating the nature of the failure.
    pub status: u16,

    /// A short, unique error code for identifying the error type.
    pub code: ErrorCode,

    /// A detailed, human-readable description of the error. Provides additional context
    /// and, if applicable, steps to resolve the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An optional URI linking to a document or resource with more information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// An array of additional error details or nested errors that occurred during the process.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_errors: Vec<HashMap<String, serde_json::Value>>,
}

impl AppError {
    pub fn new(status: u16, code: ErrorCode) -> Self {
        AppError {
            status,
            code,
            description: None,
            uri: None,
            additional_errors: Vec::new(),
        }
    }

    pub fn new_with_options(status: u16, code: ErrorCode, options: AppErrorOptions) -> Self {
        AppError {
            status,
            code,
            description: options.description,
            uri: options.uri,
            additional_errors: options.additional_errors.unwrap_or_else(Vec::new),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppErrorOptions {
    pub uri: Option<String>,
    pub description: Option<String>,
    pub additional_errors: Option<Vec<HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Serialize)]
pub struct ErrorCode(String);

impl ErrorCode {
    pub fn new(code: &str) -> Self {
        ErrorCode(format!("#ERR_{}", code))
    }
}
