use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use parking_lot::Mutex;

use crate::state::State;

pub struct Service {
    data_dir: PathBuf,
    state: Mutex<State>,
}

impl Service {
    pub fn new(data_dir: PathBuf, state: State) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self {
            data_dir,
            state: Mutex::new(state),
        })
    }
}

impl Service {
    pub fn create(&self, body: String, auth: Option<(String, String)>) -> anyhow::Result<String> {
        let mut state = self.state.lock();
        let user = match auth {
            None => None,
            Some((username, password)) => Some(
                state
                    .auth_mut(username, password)
                    .ok_or(anyhow!("Not authorized"))?,
            ),
        };
        let id = uuid::Uuid::new_v4().to_string();
        let path = self.data_dir.join(&id);
        let mut file = std::fs::File::create_new(path)?;
        file.write_all(body.as_bytes())?;

        if let Some(user) = user {
            user.paste_ids.push(id.clone());
        }

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

    pub fn delete(
        &self,
        id_to_delete: uuid::Uuid,
        username: String,
        password: String,
    ) -> anyhow::Result<()> {
        let id_to_delete = id_to_delete.to_string();
        let mut state = self.state.lock();
        let user = state
            .auth_mut(username, password)
            .ok_or(anyhow!("Not authorized"))?;
        let index = match user
            .paste_ids
            .iter()
            .enumerate()
            .find(|(_, id)| **id == id_to_delete)
        {
            None => anyhow::bail!("Paste not found"),
            Some((i, _)) => i,
        };
        std::fs::remove_file(self.data_dir.join(id_to_delete))?;
        user.paste_ids.remove(index);
        // TODO: clean up dangling entries if state serialization failed
        Ok(())
    }

    pub fn register_user(&self, username: String, password: String) -> anyhow::Result<()> {
        self.state.lock().create(username, password);
        Ok(())
    }

    pub fn list(&self, username: String, password: String) -> anyhow::Result<Vec<String>> {
        let state = self.state.lock();
        let user = state
            .auth(username, password)
            .ok_or(anyhow!("Not authorized"))?;
        Ok(user.paste_ids.iter().map(|s| s.clone()).collect())
    }

    pub fn dump_state(&self, path: &Path) -> anyhow::Result<()> {
        self.state.lock().dump(path)
    }
}
