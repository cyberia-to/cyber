use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub include_private: bool,
    pub create_stubs: bool,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("."),
            output_dir: PathBuf::from("quartz-content"),
            include_private: false,
            create_stubs: true,
            verbose: false,
        }
    }
}
