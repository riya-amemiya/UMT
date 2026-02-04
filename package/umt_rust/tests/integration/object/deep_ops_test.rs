//! Integration tests for Object manipulation functions
//!
//! Tests the interaction between deep object operations:
//! - Deep merging with selective picking
//! - Complex object transformations
//! - Nested property manipulations

use std::collections::HashMap;
use umt_rust::object::{Value, umt_has, umt_key_by, umt_merge_deep, umt_omit, umt_pick};

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut base_config_map = HashMap::new();
        base_config_map.insert("app".to_string(), Value::Object(base_app));
        base_config_map.insert("database".to_string(), Value::Object(base_database));
        let base_config = Value::Object(base_config_map);

        // Create env config (update)
        let mut env_features = HashMap::new();
        env_features.insert("logging".to_string(), Value::Bool(true));
        env_features.insert("analytics".to_string(), Value::Bool(true));

        let mut env_app = HashMap::new();
        env_app.insert("version".to_string(), Value::String("1.2.0".to_string()));
        env_app.insert("features".to_string(), Value::Object(env_features));

        let mut env_database = HashMap::new();
        env_database.insert(
            "host".to_string(),
            Value::String("prod-db.example.com".to_string()),
        );

        let mut env_config_map = HashMap::new();
        env_config_map.insert("app".to_string(), Value::Object(env_app));
        env_config_map.insert("database".to_string(), Value::Object(env_database));
        let env_config = Value::Object(env_config_map);

        // Merge configs using library function
        let merged = umt_merge_deep(&base_config, &[env_config]);

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
        // Create user data as Value objects
        let mut user1_settings = HashMap::new();
        user1_settings.insert("theme".to_string(), Value::String("dark".to_string()));
        user1_settings.insert("notifications".to_string(), Value::Bool(true));

        let mut user1_profile = HashMap::new();
        user1_profile.insert("name".to_string(), Value::String("Alice".to_string()));
        user1_profile.insert(
            "email".to_string(),
            Value::String("alice@example.com".to_string()),
        );
        user1_profile.insert("settings".to_string(), Value::Object(user1_settings));

        let mut user1 = HashMap::new();
        user1.insert("id".to_string(), Value::Int(1));
        user1.insert("profile".to_string(), Value::Object(user1_profile));

        let mut user2_settings = HashMap::new();
        user2_settings.insert("theme".to_string(), Value::String("light".to_string()));
        user2_settings.insert("notifications".to_string(), Value::Bool(false));

        let mut user2_profile = HashMap::new();
        user2_profile.insert("name".to_string(), Value::String("Bob".to_string()));
        user2_profile.insert(
            "email".to_string(),
            Value::String("bob@example.com".to_string()),
        );
        user2_profile.insert("settings".to_string(), Value::Object(user2_settings));

        let mut user2 = HashMap::new();
        user2.insert("id".to_string(), Value::Int(2));
        user2.insert("profile".to_string(), Value::Object(user2_profile));

        let user_data = vec![Value::Object(user1), Value::Object(user2)];
        let user_data_val = Value::Array(user_data);

        // Create index by id using library function
        let user_index = umt_key_by(&user_data_val, "id");

        // Pick profiles and omit email using library functions
        let user_profiles: Vec<Value> = if let Value::Object(index_map) = &user_index {
            index_map
                .values()
                .filter_map(|value| {
                    if let Value::Object(_) = value {
                        // Pass value (which is &Value) to umt_pick
                        let mut picked = umt_pick(value, &["id", "profile"]);

                        // Access profile from picked result
                        let profile_opt = if let Value::Object(ref p_map) = picked {
                            p_map.get("profile").cloned()
                        } else {
                            None
                        };

                        if let Some(profile_val) = profile_opt {
                            let profile_without_email = umt_omit(&profile_val, &["email"]);

                            // Insert modified profile back into picked
                            if let Value::Object(ref mut picked_map) = picked {
                                picked_map.insert("profile".to_string(), profile_without_email);
                            }
                            return Some(picked);
                        }
                    }
                    None
                })
                .collect()
        } else {
            vec![]
        };

        // Verify user index
        if let Some(Value::Object(user)) = user_index.get("1") {
            if let Some(Value::Object(profile)) = user.get("profile") {
                assert_eq!(
                    profile.get("name"),
                    Some(&Value::String("Alice".to_string()))
                );
            }
        }

        // Verify profiles without email
        for profile in &user_profiles {
            if let Some(p_map) = profile.as_object() {
                if let Some(Value::Object(p)) = p_map.get("profile") {
                    assert!(!p.contains_key("email"));
                    assert!(p.contains_key("name"));
                }
            }
        }
    }

    #[test]
    fn should_handle_complex_nested_merging_with_property_checking() {
        // Create default config
        let mut default_cors = HashMap::new();
        default_cors.insert("enabled".to_string(), Value::Bool(true));
        default_cors.insert(
            "origins".to_string(),
            Value::Array(vec![Value::String("http://localhost".to_string())]),
        );

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
        default_logging.insert(
            "transports".to_string(),
            Value::Array(vec![Value::String("console".to_string())]),
        );

        let mut default_features = HashMap::new();
        default_features.insert("caching".to_string(), Value::Bool(true));
        default_features.insert("logging".to_string(), Value::Object(default_logging));

        let mut default_config_map = HashMap::new();
        default_config_map.insert("server".to_string(), Value::Object(default_server));
        default_config_map.insert("features".to_string(), Value::Object(default_features));
        let default_config = Value::Object(default_config_map);

        // Create custom config
        let mut custom_cors = HashMap::new();
        custom_cors.insert(
            "origins".to_string(),
            Value::Array(vec![Value::String("https://example.com".to_string())]),
        );

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
        custom_logging.insert(
            "transports".to_string(),
            Value::Array(vec![
                Value::String("console".to_string()),
                Value::String("file".to_string()),
            ]),
        );

        let mut custom_features = HashMap::new();
        custom_features.insert("logging".to_string(), Value::Object(custom_logging));

        let mut custom_config_map = HashMap::new();
        custom_config_map.insert("server".to_string(), Value::Object(custom_server));
        custom_config_map.insert("features".to_string(), Value::Object(custom_features));
        let custom_config = Value::Object(custom_config_map);

        // Merge configs using library function
        let final_config = umt_merge_deep(&default_config, &[custom_config]);

        // Verify paths exist
        assert!(umt_has(&final_config, "server.middleware.cors.enabled"));
        assert!(umt_has(
            &final_config,
            "server.middleware.rateLimit.enabled"
        ));
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
        specs.insert(
            "materials".to_string(),
            Value::Array(vec![
                Value::String("aluminum".to_string()),
                Value::String("plastic".to_string()),
            ]),
        );

        let mut details = HashMap::new();
        details.insert(
            "name".to_string(),
            Value::String("Premium Widget".to_string()),
        );
        details.insert(
            "description".to_string(),
            Value::String("High-quality widget".to_string()),
        );
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

        let mut product_data_map = HashMap::new();
        product_data_map.insert("id".to_string(), Value::String("prod-123".to_string()));
        product_data_map.insert("details".to_string(), Value::Object(details));
        product_data_map.insert("pricing".to_string(), Value::Object(pricing));
        product_data_map.insert("inventory".to_string(), Value::Object(inventory));
        let product_data = Value::Object(product_data_map);

        // Create filtered views using library functions
        let internal_view = umt_omit(&product_data, &["pricing"]);
        let pricing_only = umt_pick(&product_data, &["id", "pricing"]);

        // Verify views
        assert!(!umt_has(&internal_view, "pricing"));
        assert!(umt_has(&pricing_only, "pricing"));
        assert!(umt_has(&pricing_only, "id"));
    }

    #[test]
    fn should_handle_deep_merging_with_array_data_and_selective_extraction() {
        // Create base permissions
        let mut base_permissions_map = HashMap::new();
        base_permissions_map.insert(
            "user".to_string(),
            Value::Array(vec![Value::String("read".to_string())]),
        );
        base_permissions_map.insert(
            "editor".to_string(),
            Value::Array(vec![
                Value::String("read".to_string()),
                Value::String("write".to_string()),
            ]),
        );
        base_permissions_map.insert(
            "admin".to_string(),
            Value::Array(vec![
                Value::String("read".to_string()),
                Value::String("write".to_string()),
                Value::String("delete".to_string()),
            ]),
        );
        let base_permissions = Value::Object(base_permissions_map);

        let mut update_permissions_map = HashMap::new();
        update_permissions_map.insert(
            "editor".to_string(),
            Value::Array(vec![
                Value::String("read".to_string()),
                Value::String("write".to_string()),
                Value::String("publish".to_string()),
            ]),
        );
        update_permissions_map.insert(
            "moderator".to_string(),
            Value::Array(vec![
                Value::String("read".to_string()),
                Value::String("moderate".to_string()),
            ]),
        );
        let update_permissions = Value::Object(update_permissions_map);

        // Merge permissions using library function
        let merged_permissions = umt_merge_deep(&base_permissions, &[update_permissions]);

        // Extract specific permissions using library functions
        let admin_permissions = umt_pick(&merged_permissions, &["admin"]);
        let non_user_permissions = umt_omit(&merged_permissions, &["user"]);

        if let Some(Value::Array(perms)) = admin_permissions.get("admin") {
            assert_eq!(perms.len(), 3);
        }

        assert!(!umt_has(&non_user_permissions, "user"));
        assert!(umt_has(&non_user_permissions, "moderator"));
    }

    #[test]
    fn should_chain_multiple_object_operations_in_complex_workflows() {
        // Create user data as Value objects
        let mut user1 = HashMap::new();
        user1.insert("id".to_string(), Value::String("user-1".to_string()));
        user1.insert("name".to_string(), Value::String("Alice".to_string()));
        user1.insert(
            "email".to_string(),
            Value::String("alice@example.com".to_string()),
        );
        user1.insert("theme".to_string(), Value::String("dark".to_string()));
        user1.insert("active".to_string(), Value::Bool(true));

        let mut user2 = HashMap::new();
        user2.insert("id".to_string(), Value::String("user-2".to_string()));
        user2.insert("name".to_string(), Value::String("Bob".to_string()));
        user2.insert(
            "email".to_string(),
            Value::String("bob@example.com".to_string()),
        );
        user2.insert("theme".to_string(), Value::String("light".to_string()));
        user2.insert("active".to_string(), Value::Bool(false));

        let api_response = vec![Value::Object(user1), Value::Object(user2)];

        // Filter active users and process using library functions
        let processed_data: Vec<Value> = api_response
            .iter()
            .filter_map(|value| {
                if let Value::Object(user) = value {
                    if user.get("active") == Some(&Value::Bool(true)) {
                        let mut picked = umt_pick(value, &["id", "name", "theme"]);

                        // Add summary
                        let mut summary = HashMap::new();
                        if let Some(Value::String(name)) = user.get("name") {
                            summary.insert("displayName".to_string(), Value::String(name.clone()));
                        }
                        summary.insert("isActive".to_string(), Value::Bool(true));

                        if let Value::Object(ref mut map) = picked {
                            map.insert("summary".to_string(), Value::Object(summary));
                        }
                        return Some(picked);
                    }
                }
                None
            })
            .collect();

        // Create user index using library function
        let user_data_values = Value::Array(processed_data.clone());
        let user_index = umt_key_by(&user_data_values, "id");

        // Merge with metadata using library function
        let mut meta = HashMap::new();
        meta.insert(
            "timestamp".to_string(),
            Value::String("2025-01-01T00:00:00Z".to_string()),
        );

        let mut users_obj = HashMap::new();
        if let Value::Object(index_map) = user_index {
            for (k, v) in index_map {
                users_obj.insert(k, v);
            }
        }

        let mut final_config_map = HashMap::new();
        final_config_map.insert("users".to_string(), Value::Object(users_obj));
        final_config_map.insert("meta".to_string(), Value::Object(meta));
        let final_config = Value::Object(final_config_map);

        assert_eq!(processed_data.len(), 1);

        // Verify final config structure
        assert!(umt_has(&final_config, "users"));
        assert!(umt_has(&final_config, "meta.timestamp"));

        if let Some(Value::Object(users)) = final_config.get("users") {
            if let Some(Value::Object(user1)) = users.get("user-1") {
                if let Some(Value::Object(summary)) = user1.get("summary") {
                    assert_eq!(
                        summary.get("displayName"),
                        Some(&Value::String("Alice".to_string()))
                    );
                }
            }
        }
    }
}
