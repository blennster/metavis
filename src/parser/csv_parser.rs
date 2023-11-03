use anyhow::{anyhow, Result};
use std::str::FromStr;

pub struct Loc {
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
}
impl FromStr for Loc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split(',');
        let start_line = splt.next().unwrap().parse()?;
        let start_col = splt.next().unwrap().parse()?;
        let end_line = splt.next().unwrap().parse()?;
        let end_col = splt.next().unwrap().parse()?;

        Ok(Loc {
            start_line,
            start_col,
            end_line,
            end_col,
        })
    }
}

pub struct DebugLoc {
    pub node_id: u32,
    pub source_file: String,
    pub loc: Loc,
}

impl FromStr for DebugLoc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split(',');
        let node_id = splt.next().ok_or(anyhow!("missing node id"))?.parse()?;
        let source_file = splt.next().ok_or(anyhow!("missing name"))?.to_owned();
        let loc = Loc::from_str(&splt.collect::<Vec<&str>>().join(",")).unwrap();

        Ok(DebugLoc {
            node_id,
            source_file,
            loc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_loc() {
        let line = "27,12,28,13";
        let loc = Loc::from_str(line).unwrap();
        assert_eq!(loc.start_line, 27);
        assert_eq!(loc.start_col, 12);
        assert_eq!(loc.end_line, 28);
        assert_eq!(loc.end_col, 13);
    }

    #[test]
    fn test_parse_debug_loc() {
        let line = "46,tests/clang/evaluation/src/arena/test1.c,27,12,28,13";
        let debug_loc = DebugLoc::from_str(line).unwrap();
        assert_eq!(debug_loc.node_id, 46);
        assert_eq!(
            debug_loc.source_file,
            "tests/clang/evaluation/src/arena/test1.c"
        );
    }
}
