use serde::{Deserialize, Serialize};
use xlog_rs::log;
mod def;
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub exts: Vec<def::daemon::ExtInfo>,
}

#[derive(Debug)]
pub struct Config {
    pub file: File,
}
impl Config {
    pub fn new() -> Self {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        let (sf, f) = match std::fs::File::open(dir.clone() + "/man.yaml") {
            Ok(f) => {
                let sf = f;
                let f: File = serde_yaml::from_reader(&sf).unwrap();
                (sf, f)
            }
            Err(_) => {
                std::fs::create_dir_all(&dir)
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
                let sf = std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(dir.clone() + "/man.yaml")
                    .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/man.yaml file"));
                let f = File { exts: Vec::new() };
                serde_yaml::to_writer(&sf, &f).unwrap_or_else(|_| {
                    todo!("Failed to create default ~/.config/qst/backend.yaml file")
                });
                (sf, f)
            }
        };
        Self { file: f }
    }
    // fn file() -> std::fs::File {
    //     let dir = std::env!("HOME").to_string() + "/.config/qst";
    //     std::fs::File::open(dir.clone() + "/backend.yaml").unwrap_or_else(|_| {
    //         std::fs::create_dir_all(&dir)
    //             .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
    //         let f = std::fs::File::create(dir.clone() + "/backend.yaml")
    //             .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
    //         serde_yaml::to_writer(
    //             &f,
    //             &File {
    //                 exts: std::collections::HashMap::new(),
    //             },
    //         )
    //         .unwrap_or_else(|_| todo!("Failed to create default ~/.config/qst/backend.yaml file"));
    //         f
    //     })
    // }
    // pub fn update(&mut self) {
    //     let dir = std::env!("HOME").to_string() + "/.config/qst";
    //     match std::fs::File::open(dir.clone() + "/backend.yaml") {
    //         Ok(f) => {
    //             if let Some(t) = self.last_update {
    //                 if t == f.metadata().unwrap().modified().unwrap() {
    //                     return;
    //                 }
    //             }
    //             self.last_update = f.metadata().unwrap().modified().map_or(None, |t| Some(t));
    //             let f: File = serde_yaml::from_reader(&f).unwrap();
    //             self.by_prompt = f
    //                 .exts
    //                 .iter()
    //                 .map(|(k, v)| (v.prompt.clone(), k.clone()))
    //                 .collect();
    //             self.file = f;
    //         }
    //         Err(_) => {
    //             std::fs::create_dir_all(&dir)
    //                 .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/ directory"));
    //             let sf = std::fs::OpenOptions::new()
    //                 .create(true)
    //                 .write(true)
    //                 .open(dir.clone() + "/backend.yaml")
    //                 .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/backend.yaml file"));
    //             let f = File {
    //                 exts: std::collections::HashMap::new(),
    //             };
    //             serde_yaml::to_writer(&sf, &f).unwrap_or_else(|_| {
    //                 todo!("Failed to create default ~/.config/qst/backend.yaml file")
    //             });
    //             self.last_update = sf.metadata().unwrap().modified().map_or(None, |t| Some(t));
    //             self.by_prompt = f
    //                 .exts
    //                 .iter()
    //                 .map(|(k, v)| (v.prompt.clone(), k.clone()))
    //                 .collect();
    //             self.file = f;
    //         }
    //     }
    //     // self.last_update = std::time::SystemTime::now();
    //     // self.by_prompt = f.exts.iter().map(|(k, v)| (v.prompt.clone(), *k)).collect();
    //     // self.file = f;
    // }
    pub fn save(&self) {
        let dir = std::env!("HOME").to_string() + "/.config/qst";
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(dir.clone() + "/man.yaml")
            .unwrap_or_else(|_| todo!("Failed to create ~/.config/qst/man.yaml file"));
        serde_yaml::to_writer(&f, &self.file)
            .unwrap_or_else(|_| todo!("Failed to create default ~/.config/qst/man.yaml file"));
    }
}
use iced::{self, widget, Application, Command};
#[derive(Debug, Clone)]
enum ExtField {
    Name(String),
    Prompt(String),
    Dir(String),
    Exec(String),
    Addr(String),
}
#[derive(Debug, Clone)]
enum Message {
    PickExt(usize),
    Input(ExtField),
}
struct App {
    pub config: Config,
    pub selected: usize,
    pub current: def::daemon::ExtInfo,
}
impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;
    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        let mut cfg = Config::new();
        // cfg.file.exts.push(def::daemon::ExtInfo {
        //     name: "test".to_string(),
        //     prompt: "test".to_string(),
        //     dir: "test".to_string(),
        //     exec: "test".to_string(),
        //     addr: "test".to_string(),
        // });
        // cfg.save();
        (
            Self {
                config: Config::new(),
                selected: 0,
                current: def::daemon::ExtInfo::default(),
            },
            iced::Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("QST-SETTINGS")
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::PickExt(idx) => {
                self.selected = idx;
            }
            Message::Input(field) => match field {
                ExtField::Name(s) => {
                    self.current.name = s;
                }
                ExtField::Prompt(s) => {
                    self.current.prompt = s;
                }
                ExtField::Dir(s) => {
                    self.current.dir = s;
                }
                ExtField::Exec(s) => {
                    self.current.exec = s;
                }
                ExtField::Addr(s) => {
                    self.current.addr = s;
                }
            },
        }
        Command::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct PickItem {
            name: String,
            idx: usize,
        }
        impl PickItem {
            fn new(name: String, idx: usize) -> Self {
                Self { name, idx }
            }
        }
        impl ToString for PickItem {
            fn to_string(&self) -> String {
                self.name.clone()
            }
        }
        let opts = self
            .config
            .file
            .exts
            .iter()
            .enumerate()
            .map(|(idx, ext)| PickItem::new(ext.name.clone(), idx))
            .collect::<Vec<_>>();
        let selected = self
            .config
            .file
            .exts
            .get(self.selected)
            .map(|ext| PickItem::new(ext.name.clone(), self.selected));
        let mut values = Vec::new();
        values.push(widget::TextInput::new("", &self.current.name).on_input(ExtField::Name));
        values.push(widget::TextInput::new("", &self.current.prompt).on_input(ExtField::Prompt));
        values.push(widget::TextInput::new("", &self.current.dir).on_input(ExtField::Dir));
        values.push(widget::TextInput::new("", &self.current.exec).on_input(ExtField::Exec));
        values.push(widget::TextInput::new("", &self.current.addr).on_input(ExtField::Addr));
        let pick =
            widget::PickList::new(opts, selected, |item: PickItem| Message::PickExt(item.idx));

        let tabs = iced::widget::Column::with_children(
            ["name", "prompt", "dir", "exec", "addr"]
                .into_iter()
                .map(|s| widget::Text::new(s).into())
                .collect(),
        );

        let values = iced::widget::Column::with_children(
            values
                .into_iter()
                .map(|e| iced::Element::new(e).map(|m| Message::Input(m)))
                .collect(),
        );
        // .width(iced::Length::Fill)
        widget::Column::new()
            // .push(widget::Text::new("Extensions"))
            .push(pick)
            .push(
                widget::Row::new()
                    .push(tabs)
                    .push(values)
                    // .width(iced::Length::Fill)
                    .align_items(iced::Alignment::Center)
                    .spacing(5),
            )
            .width(iced::Length::Fill)
            .align_items(iced::Alignment::Center)
            .into()
    }
}
fn main() {
    let s = iced::Settings::default();
    App::run(s);
}
