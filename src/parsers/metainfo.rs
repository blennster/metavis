use std::{collections::HashMap, io::BufRead, rc::Rc, str::FromStr};

use super::{
    analysis::Tuple, csv_file::RawAnalysis, debug_file::DebugJson, lib::SourceFile,
    loc_file::DebugLoc,
};

#[derive(Debug)]
pub struct MetaInfo {
    pub source_files: HashMap<String, Rc<SourceFile>>,
    pub debug_json: Vec<DebugJson>,
    pub analyses: Vec<RawAnalysis>,
    pub debug_locs: Vec<DebugLoc>,
}

impl MetaInfo {
    pub fn new(root: &str) -> Self {
        let json = std::fs::read_to_string(format!("{}/debug.json", root)).unwrap();
        let debug_json = serde_json::from_str::<Vec<DebugJson>>(&json).unwrap();
        let mut analyses = vec![];
        let mut source_files: HashMap<String, Rc<SourceFile>> = HashMap::new();
        let mut debug_loc_files: Vec<&str> = vec![];
        let mut debug_locs: Vec<DebugLoc> = vec![];

        for d in &debug_json {
            let file = std::fs::File::open(format!("{}/{}", root, d.file)).unwrap();
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                let analysis = RawAnalysis::new(&d.fields_with_nodes, &d.name, &line.unwrap());
                analyses.push(analysis);
            }

            debug_loc_files.push(&d.loc_file);
        }

        debug_loc_files.dedup();

        for d in &debug_loc_files {
            let file = std::fs::File::open(format!("{}/{}", root, d)).unwrap();
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                let loc = DebugLoc::from_str(&line.unwrap()).unwrap();

                if !source_files.contains_key(&loc.source_file) {
                    let source_file = SourceFile::new_from_path(
                        &loc.source_file,
                        format!("{}/{}", root, loc.source_file).as_str(),
                    );
                    if let Ok(source_file) = source_file {
                        source_files.insert(loc.source_file.clone(), Rc::from(source_file));
                    }
                }

                debug_locs.push(loc);
            }
        }

        // source_files.iter_mut().for_each(|(k, v)| {
        //     *v = Rc::from(std::fs::read_to_string(format!("{}/{}", root, k)).unwrap())
        // });

        Self {
            source_files,
            debug_json,
            analyses,
            debug_locs,
        }
    }

    // TODO: Make more performant
    pub fn get_analyses(&self, nodes: &[usize]) -> Vec<Tuple> {
        let raw_analyses = self
            .analyses
            .iter()
            .filter(|d| nodes.iter().any(|n| d.nodes.contains(n)))
            .collect::<Vec<_>>();

        let debug_locs = self
            .debug_locs
            .iter()
            .filter(|d| raw_analyses.iter().any(|n| n.nodes.contains(&d.node_id)))
            .collect::<Vec<_>>();

        let mut tuples = vec![];
        for d in raw_analyses {
            let locs = d
                .nodes
                .iter()
                .map(|n| match debug_locs.iter().find(|l| l.node_id == *n) {
                    Some(l) => l.loc.clone(),
                    None => panic!("node not found in debug locs"), // TODO: Fix this
                })
                .collect();

            let d = Tuple::new(d.name.clone(), d.nodes.clone(), locs);

            tuples.push(d);
        }

        tuples
    }

    // TODO: Make more performant
    pub fn get_tuples_for_relation(&self, relation: &str) -> Vec<Tuple> {
        let raw_analyses = self
            .analyses
            .iter()
            .filter(|d| d.name == relation)
            .collect::<Vec<_>>();

        let debug_locs = self
            .debug_locs
            .iter()
            .filter(|d| raw_analyses.iter().any(|n| n.nodes.contains(&d.node_id)))
            .collect::<Vec<_>>();

        let mut tuples = vec![];
        for d in raw_analyses {
            let locs = d
                .nodes
                .iter()
                .map(|n| match debug_locs.iter().find(|l| l.node_id == *n) {
                    Some(l) => l.loc.clone(),
                    None => panic!("node not found in debug locs"), // TODO: Fix this
                })
                .collect();

            let d = Tuple::new(d.name.clone(), d.nodes.clone(), locs);

            tuples.push(d);
        }

        tuples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meta_info() {
        let meta_info = MetaInfo::new("./example_data");
        assert!(!meta_info.source_files.is_empty());
        assert!(!meta_info.debug_json.is_empty());
        assert!(!meta_info.analyses.is_empty());
        dbg!(meta_info.source_files);
    }
}
