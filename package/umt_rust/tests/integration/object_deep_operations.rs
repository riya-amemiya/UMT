//! Integration tests for Object manipulation functions
//!
//! Tests the interaction between deep object operations:
//! - Deep merging with selective picking
//! - Complex object transformations
//! - Nested property manipulations
//!
//! Note: The Rust implementation has a subset of TypeScript's object manipulation
//! functions. These tests demonstrate using the available `Value` type and `umt_has`
//! function with HashMap-based object structures.

use std::collections::HashMap;
use umt_rust::object::{umt_has, Value};

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to merge two Value objects deeply
    fn merge_deep(base: &HashMap<String, Value>, update: &HashMap<String, Value>) -> HashMap<String, Value> {
        let mut result = base.clone();

        for (key, value) in update {
            match (result.get(key), value) {
                (Some(Value::Object(base_inner)), Value::Object(update_inner)) => {
                    let merged_inner = merge_deep(base_inner, update_inner);
                    result.insert(key.clone(), Value::Object(merged_inner));
                }
                _ => {
                    result.insert(key.clone(), value.clone());
                }
            }
        }

        result
    }

    /// Helper function to pick specific keys from an object
    fn pick(obj: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        for key in keys {
            if let Some(value) = obj.get(*key) {
                result.insert(key.to_string(), value.clone());
            }
        }
        result
    }

    /// Helper function to omit specific keys from an object
    fn omit(obj: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
        let mut result = obj.clone();
        for key in keys {
            result.remove(*key);
        }
        result
    }

    /// Helper function to create an index from a collection
    fn key_by<T: Clone, F: Fn(&T) -> String>(items: &[T], get_key: F) -> HashMap<String, T> {
        let mut result = HashMap::new();
        for item in items {
            let key = get_key(item);
            result.insert(key, item.clone());
        }
        result
    }

    #[test]
    fn should_merge_deep_objects_and_pick_specific_nested_properties() {
        // Create base config
        let mut base_features = HashMap::new();
        base_features.insert("auth".to_string(), Value::Bool(true));
        base_features.insert("logging".to_string(), Value::Bool(false));

        let mut base_app = HashMap::new();
        base_app.insert("name".to_string(), Value::String("MyApp".to_string()));
        base_app.insert("version".to_string(), Value::String("1.0.0".to_string()));
        base_app.insert("features".to_string(), Value::Object(base_features));

        let mut base_database = HashMap::new();
        base_database.insert("host".to_string(), Value::String("localhost".to_string()));
        base_database.insert("port".to_string(), Value::Int(5432));

        let mut base_config = HashMap::new();
        base_config.insert("app".to_string(), Value::Object(base_app));
        base_config.insert("database".to_string(), Value::Object(base_database));

        // Create env config (update)
        let mut env_features = HashMap::new();
        env_features.insert("logging".to_string(), Value::Bool(true));
        env_features.insert("analytics".to_string(), Value::Bool(true));

        let mut env_app = HashMap::new();
        env_app.insert("version".to_string(), Value::String("1.2.0".to_string()));
        env_app.insert("features".to_string(), Value::Object(env_features));

        let mut env_database = HashMap::new();
        env_database.insert("host".to_string(), Value::String("prod-db.example.com".to_string()));

        let mut env_config = HashMap::new();
        env_config.insert("app".to_string(), Value::Object(env_app));
        env_config.insert("database".to_string(), Value::Object(env_database));

        // Merge configs
        let merged = merge_deep(&base_config, &env_config);

        // Verify deep merge results
        assert!(umt_has(&merged, "app.features.auth"));
        assert!(umt_has(&merged, "app.features.logging"));
        assert!(umt_has(&merged, "app.features.analytics"));

        // Verify values
        if let Some(Value::Object(app)) = merged.get("app") {
            if let Some(Value::Object(features)) = app.get("features") {
                assert_eq!(features.get("auth"), Some(&Value::Bool(true)));
                assert_eq!(features.get("logging"), Some(&Value::Bool(true)));
                assert_eq!(features.get("analytics"), Some(&Value::Bool(true)));
            }
        }
    }

    #[test]
    fn should_transform_nested_data_structures_with_multiple_operations() {
        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        struct User {
            id: i32,
            name: String,
            email: String,
            theme: String,
            notifications: bool,
        }

        let user_data = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                theme: "dark".to_string(),
                notifications: true,
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
                theme: "light".to_string(),
                notifications: false,
            },
        ];

        // Create index by id
        let user_index = key_by(&user_data, |u| u.id.to_string());

        // Pick profiles without email
        let user_profiles: Vec<_> = user_index
            .values()
            .map(|user| {
                let mut profile = HashMap::new();
                profile.insert("id".to_string(), Value::Int(user.id as i64));
                profile.insert("name".to_string(), Value::String(user.name.clone()));
                profile.insert("theme".to_string(), Value::String(user.theme.clone()));
                profile.insert("notifications".to_string(), Value::Bool(user.notifications));
                // Omit email by not including it
                profile
            })
            .collect();

        assert_eq!(user_index.get("1").unwrap().name, "Alice");
        assert!(!umt_has(&user_profiles[0], "email"));
        assert!(umt_has(&user_profiles[0], "id"));
        assert!(umt_has(&user_profiles[0], "name"));
    }

    #[test]
    fn should_handle_complex_nested_merging_with_property_checking() {
        // Create default config
        let mut default_cors = HashMap::new();
        default_cors.insert("enabled".to_string(), Value::Bool(true));
        default_cors.insert("origins".to_string(), Value::Array(vec![Value::String("http://localhost".to_string())]));

        let mut default_auth = HashMap::new();
        default_auth.insert("enabled".to_string(), Value::Bool(false));

        let mut default_middleware = HashMap::new();
        default_middleware.insert("cors".to_string(), Value::Object(default_cors));
        default_middleware.insert("auth".to_string(), Value::Object(default_auth));

        let mut default_server = HashMap::new();
        default_server.insert("port".to_string(), Value::Int(3000));
        default_server.insert("middleware".to_string(), Value::Object(default_middleware));

        let mut default_logging = HashMap::new();
        default_logging.insert("level".to_string(), Value::String("info".to_string()));
        default_logging.insert("transports".to_string(), Value::Array(vec![Value::String("console".to_string())]));

        let mut default_features = HashMap::new();
        default_features.insert("caching".to_string(), Value::Bool(true));
        default_features.insert("logging".to_string(), Value::Object(default_logging));

        let mut default_config = HashMap::new();
        default_config.insert("server".to_string(), Value::Object(default_server));
        default_config.insert("features".to_string(), Value::Object(default_features));

        // Create custom config
        let mut custom_cors = HashMap::new();
        custom_cors.insert("origins".to_string(), Value::Array(vec![Value::String("https://example.com".to_string())]));

        let mut custom_rate_limit = HashMap::new();
        custom_rate_limit.insert("enabled".to_string(), Value::Bool(true));
        custom_rate_limit.insert("max".to_string(), Value::Int(100));

        let mut custom_middleware = HashMap::new();
        custom_middleware.insert("cors".to_string(), Value::Object(custom_cors));
        custom_middleware.insert("rateLimit".to_string(), Value::Object(custom_rate_limit));

        let mut custom_server = HashMap::new();
        custom_server.insert("port".to_string(), Value::Int(8080));
        custom_server.insert("middleware".to_string(), Value::Object(custom_middleware));

        let mut custom_logging = HashMap::new();
        custom_logging.insert("level".to_string(), Value::String("debug".to_string()));
        custom_logging.insert("transports".to_string(), Value::Array(vec![
            Value::String("console".to_string()),
            Value::String("file".to_string()),
        ]));

        let mut custom_features = HashMap::new();
        custom_features.insert("logging".to_string(), Value::Object(custom_logging));

        let mut custom_config = HashMap::new();
        custom_config.insert("server".to_string(), Value::Object(custom_server));
        custom_config.insert("features".to_string(), Value::Object(custom_features));

        // Merge configs
        let final_config = merge_deep(&default_config, &custom_config);

        // Verify paths exist
        assert!(umt_has(&final_config, "server.middleware.cors.enabled"));
        assert!(umt_has(&final_config, "server.middleware.rateLimit.enabled"));
        assert!(umt_has(&final_config, "features.logging.transports"));

        // Verify merged values
        if let Some(Value::Object(server)) = final_config.get("server") {
            if let Some(Value::Object(middleware)) = server.get("middleware") {
                if let Some(Value::Object(cors)) = middleware.get("cors") {
                    assert_eq!(cors.get("enabled"), Some(&Value::Bool(true))); // from default
                }
            }
        }
    }

    #[test]
    fn should_create_filtered_views_of_complex_objects() {
        // Create product data
        let mut dimensions = HashMap::new();
        dimensions.insert("width".to_string(), Value::Int(10));
        dimensions.insert("height".to_string(), Value::Int(5));
        dimensions.insert("depth".to_string(), Value::Int(3));

        let mut specs = HashMap::new();
        specs.insert("weight".to_string(), Value::String("2kg".to_string()));
        specs.insert("dimensions".to_string(), Value::Object(dimensions));
        specs.insert("materials".to_string(), Value::Array(vec![
            Value::String("aluminum".to_string()),
            Value::String("plastic".to_string()),
        ]));

        let mut details = HashMap::new();
        details.insert("name".to_string(), Value::String("Premium Widget".to_string()));
        details.insert("description".to_string(), Value::String("High-quality widget".to_string()));
        details.insert("specs".to_string(), Value::Object(specs));

        let mut discounts = HashMap::new();
        discounts.insert("bulk".to_string(), Value::Float(0.1));
        discounts.insert("seasonal".to_string(), Value::Float(0.05));

        let mut pricing = HashMap::new();
        pricing.insert("base".to_string(), Value::Float(99.99));
        pricing.insert("currency".to_string(), Value::String("USD".to_string()));
        pricing.insert("discounts".to_string(), Value::Object(discounts));

        let mut inventory = HashMap::new();
        inventory.insert("stock".to_string(), Value::Int(50));
        inventory.insert("warehouse".to_string(), Value::String("WH-001".to_string()));

        let mut product_data = HashMap::new();
        product_data.insert("id".to_string(), Value::String("prod-123".to_string()));
        product_data.insert("details".to_string(), Value::Object(details));
        product_data.insert("pricing".to_string(), Value::Object(pricing));
        product_data.insert("inventory".to_string(), Value::Object(inventory));

        // Create filtered views
        let internal_view = omit(&product_data, &["pricing"]);
        let pricing_only = pick(&product_data, &["id", "pricing"]);

        // Verify views
        assert!(!umt_has(&internal_view, "pricing"));
        assert!(umt_has(&pricing_only, "pricing"));
        assert!(umt_has(&pricing_only, "id"));
    }

    #[test]
    fn should_handle_deep_merging_with_array_data_and_selective_extraction() {
        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        struct UserRole {
            id: i32,
            name: String,
            roles: Vec<String>,
        }

        let base_users = vec![
            UserRole { id: 1, name: "Alice".to_string(), roles: vec!["user".to_string()] },
            UserRole { id: 2, name: "Bob".to_string(), roles: vec!["user".to_string(), "editor".to_string()] },
        ];

        let update_users = vec![
            UserRole { id: 3, name: "Charlie".to_string(), roles: vec!["admin".to_string()] },
        ];

        // Merge users
        let all_users: Vec<_> = base_users.iter().chain(update_users.iter()).cloned().collect();

        // Create permissions
        let mut base_permissions = HashMap::new();
        base_permissions.insert("user".to_string(), Value::Array(vec![Value::String("read".to_string())]));
        base_permissions.insert("editor".to_string(), Value::Array(vec![
            Value::String("read".to_string()),
            Value::String("write".to_string()),
        ]));
        base_permissions.insert("admin".to_string(), Value::Array(vec![
            Value::String("read".to_string()),
            Value::String("write".to_string()),
            Value::String("delete".to_string()),
        ]));

        let mut update_permissions = HashMap::new();
        update_permissions.insert("editor".to_string(), Value::Array(vec![
            Value::String("read".to_string()),
            Value::String("write".to_string()),
            Value::String("publish".to_string()),
        ]));
        update_permissions.insert("moderator".to_string(), Value::Array(vec![
            Value::String("read".to_string()),
            Value::String("moderate".to_string()),
        ]));

        let merged_permissions = merge_deep(&base_permissions, &update_permissions);

        // Key by role
        let users_by_role = key_by(&all_users, |u| u.roles[0].clone());

        // Extract specific permissions
        let admin_permissions = pick(&merged_permissions, &["admin"]);
        let non_user_permissions = omit(&merged_permissions, &["user"]);

        assert_eq!(all_users.len(), 3);
        assert_eq!(users_by_role.get("admin").unwrap().name, "Charlie");

        if let Some(Value::Array(perms)) = admin_permissions.get("admin") {
            assert_eq!(perms.len(), 3);
        }

        assert!(!umt_has(&non_user_permissions, "user"));
        assert!(umt_has(&non_user_permissions, "moderator"));
    }

    #[test]
    fn should_chain_multiple_object_operations_in_complex_workflows() {
        #[derive(Debug, Clone)]
        #[allow(dead_code)]
        struct ApiUser {
            id: String,
            name: String,
            email: String,
            theme: String,
            active: bool,
        }

        let api_response = vec![
            ApiUser {
                id: "user-1".to_string(),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                theme: "dark".to_string(),
                active: true,
            },
            ApiUser {
                id: "user-2".to_string(),
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
                theme: "light".to_string(),
                active: false,
            },
        ];

        // Filter active users and process
        let processed_data: Vec<HashMap<String, Value>> = api_response
            .iter()
            .filter(|user| user.active)
            .map(|user| {
                let mut picked = HashMap::new();
                picked.insert("id".to_string(), Value::String(user.id.clone()));
                picked.insert("name".to_string(), Value::String(user.name.clone()));
                picked.insert("theme".to_string(), Value::String(user.theme.clone()));

                // Add summary
                let mut summary = HashMap::new();
                summary.insert("displayName".to_string(), Value::String(user.name.clone()));
                summary.insert("isActive".to_string(), Value::Bool(user.active));
                picked.insert("summary".to_string(), Value::Object(summary));

                picked
            })
            .collect();

        // Create user index
        let mut user_index = HashMap::new();
        for user_data in &processed_data {
            if let Some(Value::String(id)) = user_data.get("id") {
                user_index.insert(id.clone(), Value::Object(user_data.clone()));
            }
        }

        // Merge with metadata
        let mut meta = HashMap::new();
        meta.insert("timestamp".to_string(), Value::String("2025-01-01T00:00:00Z".to_string()));

        let mut final_config = HashMap::new();
        final_config.insert("users".to_string(), Value::Object(user_index.clone()));
        final_config.insert("meta".to_string(), Value::Object(meta));

        assert_eq!(processed_data.len(), 1);

        // Verify final config structure
        assert!(umt_has(&final_config, "users"));
        assert!(umt_has(&final_config, "meta.timestamp"));

        if let Some(Value::Object(users)) = final_config.get("users") {
            if let Some(Value::Object(user1)) = users.get("user-1") {
                if let Some(Value::Object(summary)) = user1.get("summary") {
                    assert_eq!(summary.get("displayName"), Some(&Value::String("Alice".to_string())));
                }
            }
        }
    }
}
