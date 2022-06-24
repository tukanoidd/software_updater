use std::{fs::File, io::Write, path::PathBuf};

use eyre::*;
use serde::{Deserialize, Serialize};

use crate::{language_config::LanguageConfig, os_config::OsConfig};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub os: Option<OsConfig>,
    pub language: Option<LanguageConfig>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join("software_updater")
            .join("config.json")
    }

    pub fn read() -> Result<Self> {
        let config_path = Self::path();

        let config_str = std::fs::read_to_string(config_path)?;

        Ok(serde_json::from_str(&config_str)?)
    }

    pub fn create_default_file() -> Result<()> {
        let config_path = Self::path();

        if !config_path.exists() {
            std::fs::create_dir_all(config_path.parent().unwrap())?;

            let mut config_file = File::create(config_path)?;
            let config = Config::default();
            let config_toml = serde_json::to_string_pretty(&config)?;

            config_file.write_all(config_toml.as_bytes())?;
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            os: Some(Default::default()),
            language: Some(Default::default()),
        }
    }
}

pub mod os_config {
    use serde::{Deserialize, Serialize};

    use crate::os_config::{linux::LinuxConfig, macos::MacOsConfig, windows::WindowsConfig};

    #[derive(Serialize, Deserialize)]
    pub struct OsConfig {
        pub linux: Option<LinuxConfig>,
        pub windows: Option<WindowsConfig>,
        pub macos: Option<MacOsConfig>,
    }

    impl Default for OsConfig {
        #[inline]
        fn default() -> Self {
            Self {
                linux: Some(Default::default()),
                windows: Some(Default::default()),
                macos: Some(Default::default()),
            }
        }
    }

    pub mod linux {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct LinuxConfig {
            pub arch: Option<ArchConfig>,
            pub deb: Option<DebConfig>,
            pub rpm: Option<RpmConfig>,
            pub portage: bool,
            pub eopkg: bool,
            pub nix_channel: bool,
            pub apk: bool,
            pub snap: bool,
            pub flatpak: bool,
            pub brew: bool,
        }

        impl Default for LinuxConfig {
            #[inline]
            fn default() -> Self {
                Self {
                    arch: Some(Default::default()),
                    deb: Some(Default::default()),
                    rpm: Some(Default::default()),
                    portage: true,
                    eopkg: true,
                    nix_channel: true,
                    apk: true,
                    snap: true,
                    flatpak: true,
                    brew: true,
                }
            }
        }

        #[derive(Serialize, Deserialize)]
        pub struct ArchConfig {
            pub official: bool,
            pub aur: bool,

            pub preferred_program_official: Option<String>,
            pub preferred_program_aur: Option<String>,
        }

        impl Default for ArchConfig {
            #[inline]
            fn default() -> Self {
                Self {
                    official: true,
                    aur: true,
                    preferred_program_official: Some("pacman".to_string()),
                    preferred_program_aur: Some("paru".to_string()),
                }
            }
        }

        #[derive(Serialize, Deserialize)]
        pub struct DebConfig {
            pub preferred_program: Option<String>,
        }

        impl Default for DebConfig {
            #[inline]
            fn default() -> Self {
                Self {
                    preferred_program: Some("apt".to_string()),
                }
            }
        }

        #[derive(Serialize, Deserialize)]
        pub struct RpmConfig {
            pub preferred_program: Option<String>,
        }

        impl Default for RpmConfig {
            #[inline]
            fn default() -> Self {
                Self {
                    preferred_program: Some("dnf".to_string()),
                }
            }
        }
    }

    pub mod windows {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct WindowsConfig {
            pub choco: bool,
            pub winget: bool,
        }

        impl Default for WindowsConfig {
            #[inline]
            fn default() -> Self {
                Self {
                    choco: true,
                    winget: true,
                }
            }
        }
    }

    pub mod macos {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct MacOsConfig {
            pub brew: bool,
        }

        impl Default for MacOsConfig {
            #[inline]
            fn default() -> Self {
                Self { brew: true }
            }
        }
    }
}

pub mod language_config {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct LanguageConfig {
        pub rust: Option<RustConfig>,
        pub dart: bool,
        pub js: Option<JSConfig>,
    }

    impl Default for LanguageConfig {
        #[inline]
        fn default() -> Self {
            Self {
                rust: Some(Default::default()),
                dart: true,
                js: Some(Default::default()),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct RustConfig {
        pub rustup: bool,
        pub cargo: bool,
    }

    impl Default for RustConfig {
        #[inline]
        fn default() -> Self {
            Self {
                rustup: true,
                cargo: true,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct PythonConfig {
        pub pip2: bool,
        pub pip3: bool,
    }

    impl Default for PythonConfig {
        #[inline]
        fn default() -> Self {
            Self {
                pip2: true,
                pip3: true,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct JSConfig {
        pub npm: bool,
        pub yarn: bool,
    }

    impl Default for JSConfig {
        #[inline]
        fn default() -> Self {
            Self {
                npm: true,
                yarn: true,
            }
        }
    }
}
