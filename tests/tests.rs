#[cfg(test)]
mod packet_str_tests {
    extern crate kvptree;
    use kvptree::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_from_str() {
        let data_1 = "[ key1 [ key2 [ ] key3 \\\"val1 val1\\\" key4 val2 ] key5 val3 key6 [ key7 val4 ] ]"
            .as_bytes()
            .to_vec();
        let graph_1 = from_string(data_1).unwrap();
        let graph_2 = ValueType::LIST(HashMap::from([
            (
                "key1".to_owned(),
                ValueType::LIST(HashMap::from([
                    (
                        "key2".to_owned(),
                        ValueType::LIST(HashMap::new()),
                    ),
                    (
                        "key3".to_owned(),
                        ValueType::STRING("val1 val1".to_owned()),
                    ),
                    (
                        "key4".to_owned(),
                        ValueType::STRING("val2".to_owned()),
                    ),
                    ])),
            ),
            (
                "key5".to_owned(),
                ValueType::STRING("val3".to_owned()),
            ),
            (
                "key6".to_owned(),
                ValueType::LIST(HashMap::from([(
                    "key7".to_owned(),
                    ValueType::STRING("val4".to_owned()),
                )])),
            ),
            ]));
        assert_eq!(graph_1, graph_2)
    }

    #[test]
    fn graph_to_str_to_graph() {
        //we do not check the string as the keys and values could be in any order.
        //Instead we do it both ways. we know string to graph works becoause of the test above, so it's fine
        let graph_1 = ValueType::LIST(HashMap::from([
            (
                "key1".to_owned(),
                ValueType::LIST(HashMap::from([
                    (
                        "key2".to_owned(),
                        ValueType::LIST(HashMap::new()),
                    ),
                    (
                        "key3".to_owned(),
                        ValueType::STRING("val1 val1".to_owned()),
                    ),
                    (
                        "key4".to_owned(),
                        ValueType::STRING("val2".to_owned()),
                    ),
                    ])),
            ),
            (
                "key5".to_owned(),
                ValueType::STRING("val3".to_owned()),
            ),
            (
                "key6".to_owned(),
                ValueType::LIST(HashMap::from([(
                    "key7".to_owned(),
                    ValueType::STRING("val4".to_owned()),
                )])),
            ),
            ]));
        let string = to_string(graph_1.clone());
        let returned_graph = from_string(string).unwrap();
        assert_eq!(graph_1, returned_graph)
    }

    #[test]
    fn test_get_str() {
        let data_1 = "[ key1 [ key2 [ ] key3 \\\"val1 val1\\\" key4 val2 ] key5 val3 key6 [ key7 val4 ] ]"
            .as_bytes()
            .to_vec();
        let graph_1 = from_string(data_1).unwrap();

        assert_eq!(graph_1.get_str("key1.key3").unwrap(), "val1 val1".to_owned());
        assert_eq!(graph_1.get_str("key5").unwrap(), "val3".to_owned());
        assert!(graph_1.get_str("key2").is_err());
        assert!(graph_1.get_str("key1").is_err());
        assert!(graph_1.get_str("key1.key2").is_err());
    }

    #[test]
    fn test_get_node() {
        let data_1 = "[ key1 [ key2 [ ] key3 \\\"val1 val1\\\" key4 val2 ] key5 val3 key6 [ key7 val4 ] ]"
            .as_bytes()
            .to_vec();
        let graph_1 = from_string(data_1).unwrap();

        assert_eq!(graph_1.get_node("key1.key2").unwrap(), ValueType::LIST(HashMap::new()));
        assert_eq!(graph_1.get_node("key6").unwrap(), ValueType::LIST(HashMap::from([("key7".to_owned(), ValueType::STRING("val4".to_owned()))])));
        assert!(graph_1.get_node("key1.key3").is_err());
        assert!(graph_1.get_node("key5").is_err());
    }
}
