use derive_more::From;
use dioxus::prelude::ServerFnError;

use crate::shared::Difficulty;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Database
    FailedToLoadCourses {
        error: String,
    },
    FailedToLoadChapters {
        error: String,
    },
    FailedToLoadTopics {
        error: String,
    },
    FailedToLoadProblems {
        error: String,
    },
    FailedToLoadPrefixes {
        error: String,
    },
    FailedToUpdateRow {
        error: String,
    },
    FailedToInitializePool,

    // Static arrays
    EmptyStatic,

    // Sent data from frontend with bad formatting
    NumberOfProblemsIsZero,

    // Registry
    RegistryMutexIsPoisoned,
    NoSuchKeyExists {
        key: String,
    },
    NoSuchProblemInRegistry {
        id: String,
    },

    // Problem generation
    InvalidIntRange {
        min: i32,
        max: i32,
    },
    NoValidProblems,
    InvalidDifficultyNumber {
        difficulty: u8,
    },
    InvalidDifficultyEnum {
        expected: Difficulty,
    },

    // Translations
    InvalidTranslationKey {
        key: String,
        lang: String,
    },
    NoDescriptionForLang {
        name: String,
        lang: String,
    },
    NoQuestionForLang {
        name: String,
        lang: String,
    },
    NoAnswerForLang {
        name: String,
        lang: String,
    },
    NoSolutionForLang {
        name: String,
        lang: String,
    },

    // Loading inital resources
    FailedToLoadTranslations,

    TooManyExclusions,

    // Frontend - getting parts of course structure
    NoCourseWithCourseName {
        name: String,
    },
    NoChapterWithChapterName {
        name: String,
    },
    NoTopicWithTopicName {
        name: String,
    },

    // Frontend magic
    WebAPIFailed,
    PDFLoadingFailed,
    PDFDownloadFailed,

    // Externals
    #[from]
    Io(std::io::Error),
    #[from]
    Serde(serde_json::Error),
}

impl Into<ServerFnError> for Error {
    fn into(self) -> ServerFnError {
        return ServerFnError::ServerError { message: String::from("server error"), code: 500, details: None };
    }
}



impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// Used by the frontend to clean upp errors thrown to the ErrorBoundary
pub fn clean_error_message(error_string: String) -> String {
    // Look for "ServerError(" in the string
    if let Some(start) = error_string.find("ServerError(") {
        let start_pos = start + "ServerError(".len();

        // Find the matching closing parenthesis
        let remaining = &error_string[start_pos..];

        // Count parentheses to find the matching closing one
        let mut paren_count = 1; // Start with 1 since we're inside ServerError(
        let mut end_pos = 0;
        let mut in_string = false;
        let mut escaped = false;

        for (i, ch) in remaining.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' if in_string => escaped = true,
                '"' => in_string = !in_string,
                '(' if !in_string => paren_count += 1,
                ')' if !in_string => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        end_pos = i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if end_pos > 0 {
            let mut content = remaining[..end_pos].to_string();

            // Remove leading and trailing whitespace
            content = content.trim().to_string();

            // Remove trailing comma if present
            if content.ends_with(',') {
                content = content[..content.len() - 1].trim().to_string();
            }

            // Remove outer quotes if present
            if content.starts_with('"') && content.ends_with('"') {
                content = content[1..content.len() - 1].to_string();
            }

            // Remove backslashes (unescape)
            content = content.replace("\\\"", "\"");
            content = content.replace("\\\\", "\\");

            content
        } else {
            // Fallback if we can't find the closing parenthesis
            error_string.to_string()
        }
    } else {
        // If no ServerError found, return the original string
        error_string.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_error_message() {
        let input = r#"CapturedError { error: ServerError( "NoSuchProblemInRegistry { id: \"standard_equations_default_positiv\" }", ), backtrace: , scope: ScopeId( 12, "app::components::pdf_button::PDFButtons", ), }"#;
        let result = clean_error_message(input.to_string());
        assert_eq!(
            result,
            "NoSuchProblemInRegistry { id: \"standard_equations_default_positiv\" }".to_string()
        );

        // Test with no ServerError
        let input2 = "Some other error";
        let result2 = clean_error_message(input2.to_string());
        assert_eq!(result2, "Some other error".to_string());

        // Test with trailing comma
        let input3 = r#"ServerError( "Test error message", )"#;
        let result3 = clean_error_message(input3.to_string());
        assert_eq!(result3, "Test error message".to_string());
    }
}
