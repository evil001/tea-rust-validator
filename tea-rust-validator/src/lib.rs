// src/lib.rs

/// 检查是否是有效的邮箱地址
/// 这只是一个非常基础的检查，实际应用中可能需要更复杂的正则表达式
pub fn is_valid_email(email: &str) -> bool {
    // 检查是否包含 '@' 字符
    email.contains('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(is_valid_email("user@example.com"));
    }

    #[test]
    fn test_invalid_email() {
        assert!(!is_valid_email("useratexample.com"));
    }
}
