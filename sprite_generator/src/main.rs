use std::thread;
mod renderer;
mod time;
use renderer::{CameraSettings, Perspective, ShadingModes, SpriteRenderer, Vec3};

fn main() {
    let mut cfg_watcher = cli::CfgWatcher::new("cfg.json");
    loop {
        match cfg_watcher.poll() {
            Some(cfg) => {
                execution_pass(&cfg);
            }
            None => {
                thread::sleep(std::time::Duration::from_micros(100));
            }
        }
    }
}

fn execution_pass(cfg: &cli::RenderSettings) {
    let clock = time::Clock::new();

    let camera_settings = cfg.camera_settings;

    let image_width = cfg.image_width;
    let image_height = cfg.image_height;

    let mesh_file = cfg.mesh_file.clone();

    let mut renderer = SpriteRenderer {
        image_width,
        image_height,
        shading_mode: ShadingModes::Normal,
        camera_settings,
        mesh_file,
        blacken_normal_map: cfg.blacken_normal_map,
    };

    let img = renderer.render();
    img.save("normal.png").unwrap();

    renderer.shading_mode = ShadingModes::Diffuse;

    let img = renderer.render();
    img.save("diffuse.png").unwrap();

    println!("Run time: {:?}.", clock.elapsed());
}

mod cli {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::time::SystemTime;

    pub struct CfgWatcher {
        last_updated: Option<SystemTime>,
        file: &'static str,
    }

    impl CfgWatcher {
        pub fn new(file: &'static str) -> Self {
            Self {
                file: file,
                last_updated: None,
            }
        }

        pub fn poll(&mut self) -> Option<RenderSettings> {
            let (settings, modified_at) = {
                match from_cfg_file(self.file) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{:?}", e);
                        return None;
                    }
                }
            };
            match self.last_updated {
                Some(last_execution) => {
                    if last_execution < modified_at {
                        self.last_updated = Some(modified_at);
                        return Some(settings);
                    }
                }
                None => {
                    self.last_updated = Some(modified_at);
                    return Some(settings);
                }
            };

            None
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct RenderSettings {
        pub just_updated: Option<bool>,
        pub camera_settings: CameraSettings,
        pub image_width: u32,
        pub image_height: u32,
        pub mesh_file: String,
        pub blacken_normal_map: bool,
    }

    fn from_cfg_file(file: &'static str) -> Result<(RenderSettings, SystemTime)> {
        let path = Path::new(file);
        let mut file = File::open(path).unwrap();
        let modified = file.metadata().unwrap().modified().unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let settings: RenderSettings = serde_json::from_str(&contents)?;

        Ok((settings, modified))
    }
}
