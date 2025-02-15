mod search_criteria_tests {
    use std::collections::BTreeSet;

    use super::super::generate_search_criteria;
    use crate::AppConfig;

    // Generate search criteria with single whitelisted address and target address
    #[test]
    fn test_generate_search_criteria_single_whitelist_with_target() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter(["test@example.com".to_string()]),
            target_address: Some("target@example.com".to_string()),
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(criteria, r#"UNSEEN TO "target@example.com" ( FROM "test@example.com")"#);
    }

    // Generate search criteria with empty whitelist and no target address filter
    #[test]
    fn test_generate_search_criteria_empty_whitelist_no_target() {
        let config = AppConfig {
            whitelist: BTreeSet::new(),
            target_address: None,
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(criteria, r#"UNSEEN TO "user@example.com" ( )"#);
    }

    // Generate search criteria with multiple whitelisted addresses and target address
    #[test]
    fn test_generate_search_criteria_multiple_whitelist_with_target() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter(["test1@example.com".to_string(), "test2@example.com".to_string()]),
            target_address: Some("target@example.com".to_string()),
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(
            criteria,
            r#"UNSEEN TO "target@example.com" (OR FROM "test1@example.com" FROM "test2@example.com")"#
        );
    }

    // Generate search criteria with single whitelisted address and no target address (uses username)
    #[test]
    fn test_generate_search_criteria_single_whitelist_no_target() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter(["test@example.com".to_string()]),
            target_address: None,
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(criteria, r#"UNSEEN TO "user@example.com" ( FROM "test@example.com")"#);
    }

    // Generate search criteria with target address containing special characters
    #[test]
    fn test_generate_search_criteria_special_characters_in_target() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter(["test@example.com".to_string()]),
            target_address: Some("target+special@example.com".to_string()),
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(
            criteria,
            r#"UNSEEN TO "target+special@example.com" ( FROM "test@example.com")"#
        );
    }

    // Generate search criteria with very long whitelist addresses
    #[test]
    fn test_generate_search_criteria_long_whitelist() {
        let long_address_a = "a".repeat(1000) + "@example.com";
        let long_address_b = "b".repeat(1000) + "@example.com";
        let config = AppConfig {
            whitelist: BTreeSet::from_iter([long_address_a.clone(), long_address_b.clone()]),
            target_address: Some("target@example.com".to_string()),
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(
            criteria,
            format!(r#"UNSEEN TO "target@example.com" (OR FROM "{long_address_a}" FROM "{long_address_b}")"#,)
        );
    }

    // Generate search criteria with unicode characters in addresses, even though unicode is only allowed in the local part
    // of the email address (RFC 6531)
    #[test]
    fn test_generate_search_criteria_with_unicode_addresses() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter(["tést@exámple.com".to_string(), "üser@domäin.com".to_string()]),
            target_address: Some("tárget@exámple.com".to_string()),
            username: "usér@exámple.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(
            criteria,
            r#"UNSEEN TO "tárget@exámple.com" (OR FROM "tést@exámple.com" FROM "üser@domäin.com")"#
        );
    }

    #[test]
    fn test_generate_search_criteria_with_special_characters() {
        let config = AppConfig {
            whitelist: BTreeSet::from_iter([
                "test+filter@example.com".to_string(),
                "user.name@example.com".to_string(),
                "user_name@example.com".to_string(),
                "user-name@example.com".to_string(),
                "user@sub.example.com".to_string(),
                "user@123.123.123.123".to_string(),
                "user@[IPv6:2001:db8::1]".to_string(),
            ]),
            target_address: Some("target+special@example.com".to_string()),
            username: "user@example.com".to_string(),
            ..Default::default()
        };

        let criteria = generate_search_criteria(&config);

        assert_eq!(
            criteria,
            concat!(
                r#"UNSEEN TO "target+special@example.com" (OR FROM "test+filter@example.com" FROM "user-name@example.com" FROM "user.name@example.com" FROM "user@123.123.123.123" FROM "user@[IPv6:2001:db8::1]" FROM "user@sub.example.com" FROM "user_name@example.com")"#
            )
        );
    }
}
