use std::{collections::HashMap, io::BufRead, rc::Rc, str::FromStr};

use super::{
    csv_file::RawDiagnostic, debug_file::DebugJson, diagnostic::Diagnostic, loc_file::DebugLoc,
};

#[derive(Debug)]
pub struct MetaInfo {
    pub source_files: HashMap<String, Rc<str>>,
    pub debug_json: Vec<DebugJson>,
    pub diagnostics: Vec<RawDiagnostic>,
    pub debug_locs: Vec<DebugLoc>,
}

impl MetaInfo {
    pub fn new(root: &str) -> Self {
        let json = std::fs::read_to_string("./example_data/debug.json").unwrap();
        let debug_json = serde_json::from_str::<Vec<DebugJson>>(&json).unwrap();
        let mut diagnostics = vec![];
        let mut source_files: HashMap<String, Rc<str>> = HashMap::new();
        let mut debug_loc_files: Vec<&str> = vec![];
        let mut debug_locs: Vec<DebugLoc> = vec![];

        for d in &debug_json {
            let file = std::fs::File::open(format!("{}/{}", root, d.file)).unwrap();
            let reader = std::io::BufReader::new(file);
            let mut diags = vec![];
            for line in reader.lines() {
                let diag = RawDiagnostic::new(&d.fields_with_nodes, &d.name, &line.unwrap());
                diags.push(diag);
            }

            diagnostics.append(&mut diags);
            debug_loc_files.push(&d.loc_file);
        }

        debug_loc_files.dedup();

        for d in &debug_loc_files {
            let file = std::fs::File::open(format!("{}/{}", root, d)).unwrap();
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                let loc = DebugLoc::from_str(&line.unwrap()).unwrap();
                source_files.insert(loc.source_file.clone(), Rc::from(""));
                debug_locs.push(loc);
            }
        }

        source_files.iter_mut().for_each(|(k, v)| {
            *v = Rc::from(std::fs::read_to_string(format!("{}/{}", root, k)).unwrap())
        });

        Self {
            source_files,
            debug_json,
            diagnostics,
            debug_locs,
        }
    }

    pub fn get_diags_for_file(&self, file: &str) -> Vec<Diagnostic> {
        let nodes: Vec<_> = self
            .debug_locs
            .iter()
            .filter(|d| d.source_file == file)
            .collect();
        let raw_diags = self
            .diagnostics
            .iter()
            .filter(|d| nodes.iter().any(|n| d.nodes.contains(&n.node_id)));

        let mut diags = vec![];
        for d in raw_diags {
            let d = Diagnostic {
                name: d.name.clone(),
                source_file: file.to_string(),
                source: self.source_files[file].clone(),
                nodes: d.nodes.clone(),
                locs: nodes
                    .iter()
                    .filter(|n| d.nodes.contains(&n.node_id))
                    .map(|n| n.loc.clone())
                    .collect(),
            };

            diags.push(d);
        }

        diags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meta_info() {
        let meta_info = MetaInfo::new("./example_data");
        assert!(meta_info.source_files.len() > 0);
        assert!(meta_info.debug_json.len() > 0);
        assert!(meta_info.diagnostics.len() > 0);
        dbg!(meta_info.source_files);
    }
}
