#[derive(Default, Debug)]
pub struct SourceFile {
    pub name: String,
    pub content: String,
}

impl SourceFile {
    // pub fn new(name: &str, content: &str) -> Self {}

    pub fn new_from_path(name: &str, path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self {
            name: name.to_string(),
            content,
        })
    }
}
