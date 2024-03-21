use crate::config;

use config::Config;
use config::ExtInfo;
pub struct Server {
    config: Config,
    fuzzy: xsl::collections::FuzzyFinder<String>,
}
#[derive(serde::Serialize)]
pub struct Extheader {
    id: String,
    name: String,
}

pub type Extheaders = Vec<Extheader>;

impl Server {
    pub fn new() -> Result<Server, xcfg::Error> {
        let config = config::init()?;
        let mut fuzzy = xsl::collections::FuzzyFinder::new(usize::MAX, true);
        for (id, ext) in config.inner.exts.iter() {
            fuzzy.insert(ext.name.clone(),id.clone());
        }
        Ok(Server { config, fuzzy })
    }
    pub fn get_headers(&self, kw: String) -> Extheaders {
        println!("searching for {}", kw);
        match self.fuzzy.search_prefix(kw) {
            Some(list) => {
                println!("found {:?}", list);
                list.into_iter()
                    .map(|id| Extheader {
                        id: id.clone(),
                        name: self.config.inner.exts[id].name.clone(),
                    })
                    .collect()
            }
            None => Extheaders::new(),
        }
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
