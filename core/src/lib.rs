extern crate log;

use std::process::Command;

use enum_iterator::{all, Sequence};
use eyre::*;
use log::{debug, info};
use which::which;

pub trait PackageManagerProgram {
    fn name(&self) -> &'static str;
    fn update_instruction(&self) -> &'static str;

    fn execute_update(&self, sudo: bool) -> Result<()> {
        info!("Starting '{}' update", self.name());

        let name = self.name();
        let instructions = self.update_instruction();

        cfg_if::cfg_if! {
            if #[cfg(not(target_os = "windows"))] {
                let mut process = if sudo {
                    debug!("Executing '{} {}' with sudo", name, instructions);

                    Command::new("sudo")
                        .arg("-S")
                        .arg(name)
                        .arg(instructions)
                        .spawn()?
                } else {
                    Command::new(self.name())
                        .arg(self.update_instruction())
                        .spawn()?
                };
            } else {
                panic!("Windows is not supported yet");
            }
        }

        process.wait()?;

        info!("'{}' update is finished", self.name());

        Ok(())
    }

    #[inline]
    fn is_available(&self) -> bool {
        which(self.name()).is_ok()
    }

    fn available_programs() -> Vec<Self>
    where
        Self: Sized + Sequence,
    {
        all::<Self>()
            .filter(|program| program.is_available())
            .collect()
    }

    fn available_program(preferred_program: Option<Self>) -> Self
    where
        Self: Sized + Sequence + PartialEq + Copy + Clone,
    {
        let available_programs = Self::available_programs();

        (if let Some(preferred_program) = preferred_program {
            available_programs.iter().find_map(|program| {
                if *program == preferred_program {
                    Some(*program)
                } else {
                    None
                }
            })
        } else {
            available_programs.first().cloned()
        })
        .unwrap()
    }
}

pub trait PackageManager {
    fn get_available_program() -> Option<Box<dyn PackageManagerProgram>>;
    fn is_sudo() -> bool;

    #[inline]
    fn update(&self) -> Result<()> {
        if let Some(available_program) = Self::get_available_program() {
            available_program.execute_update(Self::is_sudo())
        } else {
            bail!("No package manager is available");
        }
    }
}

pub fn update() -> Result<()> {
    // TODO(tukanoidd): read config
    // Update os packages
    os::update()?;

    // Language packages update

    Ok(())
}

pub mod os {
    use eyre::*;
    use os_info::Type;

    use crate::PackageManager;

    pub fn update() -> Result<()> {
        // TODO(tukanoidd):
        // Parse the configuration file
        // Get right package manager based on the os
        // Update with that package manager

        #[cfg(target_os = "linux")]
        update_linux()?;

        #[cfg(target_os = "windows")]
        update_windows()?;

        #[cfg(target_os = "macos")]
        update_macos()?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn update_linux() -> Result<()> {
        let os_info = os_info::get();
        let os_type = os_info.os_type();

        match os_type {
            // Pacman
            Type::Arch | Type::EndeavourOS | Type::Manjaro | Type::Garuda => {
                use linux::pacman::{Pacman, PacmanAurUsage};

                // TODO(tukanoidd): get data from config
                Pacman {
                    aur_usage: PacmanAurUsage::WithAur,
                }
                .update()?;
            }

            // Apt
            Type::Debian | Type::Mint | Type::Pop | Type::Raspbian | Type::Ubuntu => {
                use linux::deb::Deb;

                Deb.update()?;
            }

            // Dnf/Yum
            Type::Fedora
            | Type::CentOS
            | Type::OracleLinux
            | Type::Redhat
            | Type::RedHatEnterprise => {
                bail!("Dnf/Yum is not supported yet");
            }

            // Zypper
            Type::openSUSE | Type::SUSE => {
                bail!("Zypper is not supported yet");
            }

            // Portage
            Type::Gentoo => {
                bail!("Portage is not supported yet");
            }

            // Eopkg
            Type::Solus => {
                bail!("Eopkg is not supported yet");
            }

            // Nix-channel
            Type::NixOS => {
                bail!("Nix-channel is not supported yet");
            }

            // Apk
            Type::Alpine => {
                bail!("Apk is not supported yet");
            }

            _ => bail!("{} is unsupported", os_type),
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn update_windows() -> Result<()> {
        bail!("Windows is not supported yet");
    }

    #[cfg(target_os = "macos")]
    fn update_macos() -> Result<()> {
        bail!("MacOS is not supported yet");
    }

    pub mod linux {
        pub mod pacman {
            use enum_iterator::Sequence;

            use crate::{PackageManager, PackageManagerProgram};

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
            pub enum AurProgram {
                Pamac,
                Yay,
                Paru,
            }

            impl PackageManagerProgram for AurProgram {
                fn name(&self) -> &'static str {
                    match self {
                        AurProgram::Pamac => "pamac",
                        AurProgram::Yay => "yay",
                        AurProgram::Paru => "paru",
                    }
                }

                fn update_instruction(&self) -> &'static str {
                    match self {
                        AurProgram::Pamac => "upgrade",
                        AurProgram::Yay | AurProgram::Paru => "-Sua",
                    }
                }
            }

            pub enum PacmanAurUsage {
                NoAur,
                OnlyAur,
                WithAur,
            }

            impl PacmanAurUsage {
                pub fn use_pacman_aur(&self) -> (bool, bool) {
                    match self {
                        PacmanAurUsage::NoAur => (true, false),
                        PacmanAurUsage::OnlyAur => (false, true),
                        PacmanAurUsage::WithAur => (true, true),
                    }
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
            pub enum PacmanProgram {
                Pacman,
            }

            impl PackageManagerProgram for PacmanProgram {
                #[inline]
                fn name(&self) -> &'static str {
                    "pacman"
                }

                #[inline]
                fn update_instruction(&self) -> &'static str {
                    "-Syu"
                }
            }

            pub struct Pacman {
                pub aur_usage: PacmanAurUsage,
            }

            impl PackageManager for Pacman {
                fn get_available_program() -> Option<Box<dyn PackageManagerProgram>> {
                    None
                }

                fn is_sudo() -> bool {
                    true
                }

                fn update(&self) -> eyre::Result<()> {
                    let (pacman, aur) = self.aur_usage.use_pacman_aur();

                    if pacman {
                        PacmanProgram::Pacman.execute_update(true)?;
                    }

                    if aur {
                        // TODO(tukanoidd): get data from config
                        AurProgram::available_program(Some(AurProgram::Paru))
                            .execute_update(false)?;
                    }

                    Ok(())
                }
            }
        }

        pub mod deb {
            use enum_iterator::Sequence;

            use crate::{PackageManager, PackageManagerProgram};

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
            pub enum DebProgram {
                Apt,
                Aptitude,
            }

            impl PackageManagerProgram for DebProgram {
                fn name(&self) -> &'static str {
                    match self {
                        DebProgram::Apt => "apt",
                        DebProgram::Aptitude => "aptitude",
                    }
                }

                fn update_instruction(&self) -> &'static str {
                    "update"
                }
            }

            pub struct Deb;

            impl PackageManager for Deb {
                fn get_available_program() -> Option<Box<dyn PackageManagerProgram>> {
                    // TODO(tukanoidd): get data from config
                    Some(Box::new(DebProgram::available_program(Some(
                        DebProgram::Aptitude,
                    ))))
                }

                fn is_sudo() -> bool {
                    true
                }
            }
        }
    }

    pub mod windows {}

    pub mod macos {}
}

pub mod languages {
    pub mod rust {}

    pub mod dart {}

    pub mod python {}

    pub mod go {}
}
