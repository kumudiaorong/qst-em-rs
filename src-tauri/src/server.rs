use crate::config;

use config::Config;
use config::ExtInfo;
pub struct Server {
    pub config: Config,
}
#[derive(serde::Serialize)]
pub struct Extheader {
    id: String,
    name: String,
}

pub type Extheaders = Vec<Extheader>;

impl Server {
    pub fn get_headers(&self) -> Extheaders {
        self.config
            .inner
            .exts
            .iter()
            .map(|(id, ext)| Extheader {
                id: id.clone(),
                name: ext.name.clone(),
            })
            .collect()
    }
    pub fn get_ext(&self, id: &str) -> Option<&ExtInfo> {
        self.config.inner.exts.get(id)
    }
    pub fn set_ext(&mut self, id: &str, new_ext: ExtInfo) -> Result<String, String> {
        if let Some(ext) = self.config.inner.exts.get_mut(id) {
            *ext = new_ext.clone();
            self.config.save().unwrap();
            return Ok(id.to_string());
        }
        let id = uuid::Uuid::new_v4().to_string();
        self.config.inner.exts.insert(id.clone(), new_ext);
        self.config.save().map_err(|e| e.message)?;
        Ok(id)
    }
    pub fn del_ext(&mut self, id: &str) -> Result<(), String> {
        self.config.inner.exts.remove(id);
        self.config.save().map_err(|e| e.message)?;
        Ok(())
    }
}
pub fn init() -> Result<Server, xcfg::Error> {
    let server = Server {
        config: config::init()?,
    };
    Ok(server)
}
