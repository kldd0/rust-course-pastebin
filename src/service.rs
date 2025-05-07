use std::{
    io::{Read, Write},
    path::PathBuf,
};

pub struct Service {
    data_dir: PathBuf,
}

impl Service {
    pub fn new(data_dir: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self { data_dir })
    }
}

impl Service {
    pub fn create(&self, body: String) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let path = self.data_dir.join(&id);
        let mut file = std::fs::File::create_new(path)?;
        file.write_all(body.as_bytes())?;
        Ok(id)
    }

    // TODO: stream the result instead of loading in memory
    pub fn read(&self, id: &uuid::Uuid) -> anyhow::Result<String> {
        let path = self.data_dir.join(id.to_string());
        let mut file = std::fs::File::open(path)?;
        let mut result = String::new();
        file.read_to_string(&mut result)?;
        Ok(result)
    }
}
