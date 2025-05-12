use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::state::State;

pub struct Service {
    data_dir: PathBuf,
    state: State,
}

impl Service {
    pub fn new(data_dir: PathBuf, state: State) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self { data_dir, state })
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

    pub fn register_user(&mut self, username: String, password: String) -> anyhow::Result<()> {
        self.state.create(username, password);
        Ok(())
    }

    pub fn list(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<impl Iterator<Item = &str>> {
        let user = self
            .state
            .auth(username, password)
            .ok_or(anyhow!("Not authorized"))?;
        Ok(user.paste_ids.iter().map(|s| s.as_str()))
    }

    pub fn dump_state(&self, path: &Path) -> anyhow::Result<()> {
        self.state.dump(path)
    }
}
