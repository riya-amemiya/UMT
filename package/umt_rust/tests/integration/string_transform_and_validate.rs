//! Integration tests for String transformation and validation functions
//!
//! Tests the interaction between string manipulation and validation:
//! - Base64 encoding/decoding with validation
//! - Format transformations with validation
//! - String cleaning and validation workflows

use regex::Regex;
use umt_rust::string::{
    umt_camel_case, umt_from_base64, umt_kebab_case, umt_to_base64, umt_trim_characters,
};
use umt_rust::validate::string::{
    umt_max_length, umt_min_length, umt_regex_match, umt_uuid, umt_validate_email_validator,
    umt_validate_string,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_decode_base64_and_validate_the_results() {
        let original_text = "Hello, World!";
        let encoded = umt_to_base64(original_text);
        let decoded = umt_from_base64(&encoded).unwrap();

        let base64_pattern = Regex::new(r"^[A-Za-z0-9+/=]+$").unwrap();
        let base64_validator = umt_regex_match(base64_pattern, None);

        assert!((base64_validator.validate)(&encoded));
        assert_eq!(decoded, original_text);
    }

    #[test]
    fn should_format_strings_and_validate_email_addresses() {
        // Simulating formatString by concatenation
        let generated_email = format!("{}@{}.{}", "user", "example", "com");

        let email_validator = umt_validate_email_validator(None, None);
        let result = (email_validator.validate)(&generated_email);

        assert!(result);
        assert_eq!(generated_email, "user@example.com");
    }

    #[test]
    fn should_clean_strings_and_validate_length_constraints() {
        let messy_string = "!!!Hello World!!!";
        let cleaned = umt_trim_characters(messy_string, "!");

        let min_validator = umt_min_length(5, None);
        let max_validator = umt_max_length(15, None);

        let min_valid = (min_validator.validate)(&cleaned);
        let max_valid = (max_validator.validate)(&cleaned);

        assert!(min_valid);
        assert!(max_valid);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn should_transform_case_and_validate_format() {
        let original_string = "user-profile-settings";
        let camel_cased = umt_camel_case(original_string);
        let kebab_cased = umt_kebab_case(&camel_cased);

        let camel_case_pattern = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();
        let kebab_case_pattern = Regex::new(r"^[a-z]+(-[a-z]+)*$").unwrap();

        let camel_validator = umt_regex_match(camel_case_pattern, None);
        let kebab_validator = umt_regex_match(kebab_case_pattern, None);

        assert!(
            (camel_validator.validate)(&camel_cased),
            "camelCase validation failed for: {}",
            camel_cased
        );
        assert!(
            (kebab_validator.validate)(&kebab_cased),
            "kebab-case validation failed for: {}",
            kebab_cased
        );
        assert_eq!(kebab_cased, original_string);
    }

    #[test]
    fn should_validate_uuid_after_string_manipulation() {
        // Simulating formatString for UUID
        let generated_uuid = format!(
            "{}-{}-{}-{}-{}",
            "550e8400", "e29b", "41d4", "a716", "446655440000"
        );

        let uuid_validator = umt_uuid(None, None);
        let result = (uuid_validator.validate)(&generated_uuid);

        assert!(result, "UUID validation failed for: {}", generated_uuid);
    }

    #[test]
    fn should_handle_complex_string_workflows_with_multiple_validations() {
        let raw_email = "  john.doe@EXAMPLE.COM  ";

        let trimmed = raw_email.trim();
        let lowercased = trimmed.to_lowercase();

        let email_validator = umt_validate_email_validator(None, None);
        let email_valid = (email_validator.validate)(&lowercased);

        if email_valid {
            let parts: Vec<&str> = lowercased.split('@').collect();
            let user = parts[0];
            let domain = parts[1];
            let formatted = format!("User: {}, Domain: {}", user, domain);

            let min_validator = umt_min_length(10, None);
            let formatted_valid = (min_validator.validate)(&formatted);

            assert!(formatted_valid);
            assert_eq!(formatted, "User: john.doe, Domain: example.com");
        }
    }

    #[test]
    fn should_validate_base64_encoded_json_strings() {
        let json_data = r#"{"user": "test", "id": 123}"#;
        let encoded = umt_to_base64(json_data);

        let base64_pattern = Regex::new(r"^[A-Za-z0-9+/=]+$").unwrap();
        let base64_validator = umt_regex_match(base64_pattern, None);
        assert!((base64_validator.validate)(&encoded));

        let decoded = umt_from_base64(&encoded).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&decoded).unwrap();

        assert_eq!(parsed["user"], "test");
        assert_eq!(parsed["id"], 123);
    }

    #[test]
    fn should_transform_and_validate_configuration_keys() {
        let config_keys = [
            "database-connection-string",
            "api-key-value",
            "max-retry-attempts",
        ];

        let camel_case_pattern = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();

        let transformed_keys: Vec<_> = config_keys
            .iter()
            .map(|key| {
                let camel = umt_camel_case(key);
                let camel_validator = umt_regex_match(camel_case_pattern.clone(), None);
                let validated = (camel_validator.validate)(&camel);
                (key.to_string(), camel, validated)
            })
            .collect();

        for (original, camel, validated) in &transformed_keys {
            assert!(validated, "Validation failed for: {}", camel);
            assert_eq!(umt_kebab_case(camel), *original);
        }
    }

    #[test]
    fn should_handle_string_validation_with_multiple_rules() {
        let test_strings = [
            "hello",
            "hi",
            "this is a very long string that exceeds the maximum length",
            "",
        ];

        let validators = vec![
            umt_min_length(3, Some("Too short".to_string())),
            umt_max_length(20, Some("Too long".to_string())),
        ];

        let results: Vec<_> = test_strings
            .iter()
            .map(|s| {
                let result = umt_validate_string(s, &validators, None);
                (s.to_string(), result.validate, result.message)
            })
            .collect();

        assert!(results[0].1, "hello should pass validation");
        assert!(!results[1].1, "hi should fail min length");
        assert!(!results[2].1, "long string should fail max length");
        assert!(!results[3].1, "empty string should fail min length");
    }

    #[test]
    fn should_validate_and_transform_urls() {
        let url_paths = [
            "user-profile-settings",
            "api-v2-endpoints",
            "auth-callback-handler",
        ];

        let camel_case_pattern = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();

        for path in url_paths {
            let camel = umt_camel_case(path);
            let validator = umt_regex_match(camel_case_pattern.clone(), None);
            assert!(
                (validator.validate)(&camel),
                "Failed to validate camelCase: {}",
                camel
            );

            // Round-trip transformation
            let back_to_kebab = umt_kebab_case(&camel);
            assert_eq!(back_to_kebab, path, "Round-trip failed for: {}", path);
        }
    }

    #[test]
    fn should_handle_encoding_special_characters() {
        let special_strings = [
            "Hello World!",
            "Test@123",
            "User-Name_Value",
            "Special: !@#$%^&*()",
        ];

        for original in special_strings {
            let encoded = umt_to_base64(original);
            let decoded = umt_from_base64(&encoded).unwrap();

            assert_eq!(
                decoded, original,
                "Encoding/decoding failed for: {}",
                original
            );
        }
    }

    #[test]
    fn should_combine_trimming_and_case_conversion() {
        let input = "  --HELLO--WORLD--  ";

        // Trim whitespace and dashes
        let trimmed = umt_trim_characters(input.trim(), "-");
        let lowercased = trimmed.to_lowercase();

        // Convert to different cases
        let camel = umt_camel_case(&lowercased);

        assert_eq!(trimmed, "HELLO--WORLD");
        assert_eq!(camel, "helloWorld");
    }
}
