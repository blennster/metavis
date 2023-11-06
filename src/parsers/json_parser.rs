use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DebugJson {
    name: String,
    file: String,
    locs: Vec<usize>,
    #[serde(rename = "locFile")]
    loc_file: String,
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
        assert_eq!(debug_info[0].locs, vec![0, 1, 2]);
        assert_eq!(debug_info[0].loc_file, "DEBUG_Loc.csv");
    }
}
