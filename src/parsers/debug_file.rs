use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct DebugJson {
    pub name: String,
    pub file: String,
    #[serde(rename = "locs")]
    pub fields_with_nodes: Vec<usize>,
    #[serde(rename = "locFile")]
    pub loc_file: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_debug_loc() {
        let json = r#"
        [
            {
                "name": "Call",
                "file": "Call.csv",
                "locs": [0, 1, 2],
                "locFile": "DEBUG_Loc.csv"
            }
        ]
        "#;
        let debug_info = serde_json::from_str::<Vec<DebugJson>>(json).unwrap();

        assert_eq!(debug_info[0].name, "Call");
        assert_eq!(debug_info[0].file, "Call.csv");
        assert_eq!(debug_info[0].fields_with_nodes, vec![0, 1, 2]);
        assert_eq!(debug_info[0].loc_file, "DEBUG_Loc.csv");
    }
}
