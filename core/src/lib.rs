#[macro_use]
extern crate log;

use eyre::*;
use os_info::Type;

use software_updater_config::{
    language_config::LanguageConfig, os_config::linux::LinuxConfig, Config,
};

use crate::{
    language::{update_js, update_rust},
    os::linux::{update_arch, update_deb},
};

pub fn update() {
    Config::create_default_file().unwrap();
    let Config { os, language } = Config::read().unwrap();

    if let Some(os_config) = os {
        #[cfg(target_os = "linux")]
        if let Some(LinuxConfig {
            arch,
            deb,
            rpm,
            portage,
            eopkg,
            nix_channel,
            apk,
            snap,
            flatpak,
            brew,
        }) = os_config.linux
        {
            let os_info = os_info::get();
            let os_type = os_info.os_type();

            match os_type {
                // Arch
                Type::Arch | Type::EndeavourOS | Type::Manjaro | Type::Garuda => {
                    if let Some(arch) = arch {
                        update_arch(arch);
                    }
                }

                // Apt
                Type::Debian | Type::Mint | Type::Pop | Type::Raspbian | Type::Ubuntu => {
                    if let Some(deb) = deb {
                        update_deb(deb);
                    }
                }

                // Dnf/Yum
                Type::Fedora
                | Type::CentOS
                | Type::OracleLinux
                | Type::Redhat
                | Type::RedHatEnterprise => {
                    if let Some(rpm) = rpm {
                        error!("Dnf/Yum is not supported yet");
                    }
                }

                // Zypper
                Type::openSUSE | Type::SUSE => {
                    if let Some(rpm) = rpm {
                        error!("Zypper is not supported yet");
                    }
                }

                // Portage
                Type::Gentoo => {
                    if portage {
                        error!("Portage is not supported yet");
                    }
                }

                // Eopkg
                Type::Solus => {
                    if eopkg {
                        error!("Eopkg is not supported yet");
                    }
                }

                // Nix-channel
                Type::NixOS => {
                    if nix_channel {
                        error!("Nix-channel is not supported yet");
                    }
                }

                // Apk
                Type::Alpine => {
                    if apk {
                        error!("Apk is not supported yet");
                    }
                }

                _ => error!("{} is unsupported", os_type),
            }

            if snap {
                error!("Snap is not supported yet");
            }

            if flatpak {
                error!("Flatpak is not supported yet");
            }

            if brew {
                error!("Brew is not supported yet");
            }
        }

        #[cfg(target_os = "windows")]
        if let Some(windows) = os_config.windows {
            error!("Windows is not supported yet");
        }

        #[cfg(target_os = "macos")]
        if let Some(macos) = os_config.macos {
            error!("MacOS is not supported yet");
        }
    }

    if let Some(LanguageConfig { rust, dart, js }) = language {
        if let Some(rust) = rust {
            update_rust(rust);
        }

        if dart {
            info!("Starting Flutter update!");
            if which::which("flutter").is_ok() {
                execute_update(false, "flutter", &["upgrade"]);
                execute_update(false, "flutter", &["update-packages"]);
            } else {
                error!("cargo is not installed");
            }
            info!("Finished Flutter update!");
        }

        if let Some(js) = js {
            update_js(js);
        }
    }
}

pub fn available_program(programs: &[&str], preferred: Option<String>) -> Result<String> {
    let mut program = None;

    if let Some(preferred) = preferred {
        if programs.contains(&preferred.as_str()) {
            program = Some(preferred);
        }
    }

    if let Some(program) = program {
        Ok(program)
    } else {
        programs
            .iter()
            .find_map(|program| {
                if which::which(program).is_ok() {
                    Some(program.to_string())
                } else {
                    None
                }
            })
            .wrap_err(format!(
                "No supported program available in the system, supported_programs: {:?}",
                programs,
            ))
    }
}

pub fn execute_update(sudo: bool, program_name: &str, program_args: &[&str]) {
    let full_command = format!("{} {}", program_name, program_args.join(" "));
    info!("Executing `{}`", full_command);

    let child = if sudo {
        let mut command = runas::Command::new(program_name);

        if !program_args.is_empty() {
            command.args(program_args);
        }

        command.spawn()
    } else {
        let mut command = std::process::Command::new(program_name);

        if !program_args.is_empty() {
            command.args(program_args);
        }

        command.spawn()
    }
    .map(Some)
    .unwrap_or(None);

    let mut child = match child {
        Some(child) => child,
        None => {
            error!("Failed to execute `{}` update", program_name);
            return;
        }
    };

    match child.wait() {
        Ok(exit_status) => {
            if !exit_status.success() {
                error!("Execution wasn't successful");
            }
        }
        Err(err) => {
            error!(
                "Something went wrong with the child process during execution: {}",
                err
            );
        }
    }

    info!("Execution of `{}` finished", full_command);
}

pub mod os {
    pub mod linux {
        use software_updater_config::os_config::linux::{ArchConfig, DebConfig};

        use crate::{available_program, execute_update};

        pub fn update_arch(config: ArchConfig) {
            info!("Starting Arch update!");

            let ArchConfig {
                official,
                aur,
                preferred_program_official,
                preferred_program_aur,
            } = config;

            let update_official = |program: &str| match program {
                "pacman" => execute_update(true, program, &["-Syu"]),
                "pamac" => execute_update(true, program, &["upgrade"]),
                _ => error!("Unknown command to execute: {}", program),
            };

            let update_aur = |program: &str| match program {
                "paru" => execute_update(false, program, &["-Sua"]),
                "yay" => execute_update(false, program, &["-Sua"]),
                _ => error!("Unknown command to execute: {}", program),
            };

            let update_official_and_aur = |program: &str| match program {
                "pamac" => execute_update(false, program, &["upgrade", "--aur --devel"]),
                "paru" => execute_update(false, program, &["-Syu"]),
                "yay" => execute_update(false, program, &["-Syu"]),
                _ => error!("Unknown command to execute: {}", program),
            };

            let official_and_aur = official && aur;

            if official_and_aur {
                const SUPPORTED_PROGRAMS: &[&str] = &["pamac", "yay", "paru"];

                let program = match available_program(SUPPORTED_PROGRAMS, preferred_program_aur) {
                    Ok(program) => program,
                    Err(err) => {
                        error!("Unable to find a supported program: {}", err);
                        return;
                    }
                };

                update_official_and_aur(&program);
            } else {
                if official {
                    const SUPPORTED_PROGRAMS: &[&str] = &["pacman", "pamac"];

                    let program =
                        match available_program(SUPPORTED_PROGRAMS, preferred_program_official) {
                            Ok(program) => program,
                            Err(err) => {
                                error!("Unable to find supported program: {}", err);
                                return;
                            }
                        };

                    update_official(&program);
                }

                if aur {
                    const SUPPORTED_PROGRAMS: &[&str] = &["yay", "paru"];

                    let program = match available_program(SUPPORTED_PROGRAMS, preferred_program_aur)
                    {
                        Ok(program) => program,
                        Err(err) => {
                            error!("Unable to find available program: {}", err);
                            return;
                        }
                    };

                    update_aur(&program);
                }
            };

            info!("Finished Arch update!");
        }

        pub fn update_deb(config: DebConfig) {
            info!("Starting Deb update!");

            let DebConfig { preferred_program } = config;

            const SUPPORTED_PROGRAMS: &[&str] = &["apt", "aptitude"];
            let program = match available_program(SUPPORTED_PROGRAMS, preferred_program) {
                Ok(program) => program,
                Err(err) => {
                    error!("Unable to find a supported program: {}", err);
                    return;
                }
            };

            let program_args = match program.as_str() {
                "apt" => &["upgrade"],
                "aptitude" => &["upgrade"],
                _ => {
                    error!("Unknown command to execute: {}", program);
                    return;
                }
            };

            execute_update(true, &program, program_args);

            info!("Finished Deb update!");
        }
    }

    pub mod windows {}

    pub mod macos {}
}

pub mod language {
    use software_updater_config::language_config::{JSConfig, RustConfig};

    use crate::execute_update;

    pub fn update_rust(config: RustConfig) {
        info!("Starting Rust update!");

        let RustConfig { rustup, cargo } = config;

        if rustup {
            if which::which("rustup").is_ok() {
                execute_update(false, "rustup", &["update"]);
            } else {
                error!("rustup is not installed");
            }
        }

        if cargo {
            if which::which("cargo").is_ok() {
                execute_update(false, "cargo", &["install-update", "-a"]);
            } else {
                error!("cargo is not installed");
            }
        }

        info!("Finished Rust update!");
    }

    pub fn update_js(config: JSConfig) {
        info!("Starting Rust update!");

        let JSConfig { npm, yarn } = config;

        if npm {
            if which::which("npm").is_ok() {
                execute_update(true, "npm", &["-g", "upgrade"]);
            } else {
                error!("npm is not installed");
            }
        }

        if yarn {
            if which::which("yarn").is_ok() {
                execute_update(false, "yarn", &["global", "upgrade"]);
            } else {
                error!("yarn is not installed");
            }
        }

        info!("Finished Rust update!");
    }
}
