pub struct CsvFile {
    name: String,
    nodes: Vec<usize>,
    rest: String,
}

impl CsvFile {
    pub fn new(n: Vec<usize>, name: &str, s: &str) -> Self {
        let splt = s.split(',');
        let mut nodes = vec![];
        let mut rest = vec![];

        for (i, s) in splt.enumerate() {
            if n.contains(&i) {
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
