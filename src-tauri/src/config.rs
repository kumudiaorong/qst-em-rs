use xcfg::File as XFile;

mod inner {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ExtInfo {
        pub name: String,
        pub prompt: String,
        pub dir: String,
        pub exec: String,
        pub addr: String,
    }
    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Info {
        pub exts: HashMap<String, ExtInfo>,
    }
}
pub use inner::ExtInfo;
pub type Config = XFile<inner::Info>;

pub fn init() -> Result<Config, xcfg::Error> {
    let path = dirs::home_dir().unwrap().join(".config/qst/backend.toml");
    let path = path.to_str().unwrap();
    let mut file: XFile<inner::Info> = XFile::new().path(path);
    file.load()?;
    file.save()?;
    Ok(file)
}
