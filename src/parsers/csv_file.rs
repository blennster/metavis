#[derive(Debug, Clone)]
pub struct RawDiagnostic {
    pub name: String,
    pub nodes: Vec<usize>,
    pub rest: String,
}

impl RawDiagnostic {
    pub fn new(fields_with_nodes: &[usize], name: &str, s: &str) -> Self {
        let splt = s.split(',');
        let mut nodes = vec![];
        let mut rest = vec![];

        for (i, s) in splt.enumerate() {
            if fields_with_nodes.contains(&i) {
                nodes.push(s.parse().unwrap());
            } else {
                rest.push(s);
            }
        }

        Self {
            name: String::from(name),
            nodes,
            rest: rest.join(","),
        }
    }
}
