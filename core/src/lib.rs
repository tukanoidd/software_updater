#[macro_use]
extern crate log;

use enum_iterator::{all, Sequence};
use eyre::*;
use which::which;

pub trait PackageManagerProgram {
    fn name(&self) -> &'static str;
    fn update_instructions(&self) -> &[&'static str];
    fn is_sudo(&self) -> bool;

    fn execute_update(&self) -> Result<()> {
        info!("Starting '{}' update", self.name());

        let name = self.name();
        let instructions = self.update_instructions();

        if self.is_sudo() {
            let mut process = runas::Command::new(name).args(instructions).spawn()?;

            process.wait()?;
        } else {
            let mut process = std::process::Command::new(name)
                .args(instructions)
                .spawn()?;

            process.wait()?;
        }

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

    #[inline]
    fn update(&self) -> Result<()> {
        if let Some(available_program) = Self::get_available_program() {
            available_program.execute_update()
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
                use linux::pacman::Pacman;

                Pacman.update()?;
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
            pub enum PacmanProgram {
                Paru,
                Yay,
                PamacWithAur,
                Pamac,
                Pacman,
            }

            impl PackageManagerProgram for PacmanProgram {
                #[inline]
                fn name(&self) -> &'static str {
                    use PacmanProgram::*;

                    match self {
                        PacmanProgram::Pacman => "pacman",
                        Pamac | PamacWithAur => "pamac",
                        Yay => "yay",
                        Paru => "paru",
                    }
                }

                #[inline]
                fn update_instructions(&self) -> &[&'static str] {
                    use PacmanProgram::*;

                    match self {
                        Pamac => &["upgrade"],
                        PamacWithAur => &["upgrade", "-a"],
                        Pacman | Yay | Paru => &["-Syu"],
                    }
                }

                fn is_sudo(&self) -> bool {
                    use PacmanProgram::*;

                    match self {
                        Pamac | Pacman => true,
                        PamacWithAur | Yay | Paru => false,
                    }
                }
            }

            pub struct Pacman;

            impl PackageManager for Pacman {
                #[inline]
                fn get_available_program() -> Option<Box<dyn PackageManagerProgram>> {
                    // TODO(tukanoidd): config
                    Some(Box::new(PacmanProgram::available_program(Some(
                        PacmanProgram::Paru,
                    ))))
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

                fn update_instructions(&self) -> &[&'static str] {
                    &["update"]
                }

                #[inline]
                fn is_sudo(&self) -> bool {
                    true
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
