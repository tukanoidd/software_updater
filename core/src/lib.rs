#[macro_use]
extern crate log;

use downcast_rs::{impl_downcast, Downcast};
use enum_iterator::{all, Sequence};
use eyre::*;
use which::which;

impl_downcast!(PackageManagerProgram);

pub trait PackageManagerProgram: Downcast {
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

    fn available_program(preferred_program: Option<Self>) -> Result<Self>
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
        .ok_or(eyre!("No available package manager program"))
    }
}

pub trait PackageManager {
    fn get_preferred_program() -> Box<dyn PackageManagerProgram>;
    fn get_available_program() -> Result<Box<dyn PackageManagerProgram>>;

    #[inline]
    fn update() -> Result<()> {
        Self::get_available_program()?.execute_update()
    }
}

pub fn update() -> Result<()> {
    // TODO(tukanoidd): read config
    // Update os packages
    os::update()?;

    // Language packages update
    languages::update()?;

    Ok(())
}

pub mod os {
    use eyre::*;
    use lazy_static::lazy_static;

    use crate::PackageManager;

    lazy_static! {
        static ref OS_INFO: os_info::Info = os_info::get();
    }

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
        use os_info::Type;

        let os_type = OS_INFO.os_type();

        match os_type {
            // Pacman
            Type::Arch | Type::EndeavourOS | Type::Manjaro | Type::Garuda => {
                use linux::pacman::Pacman;

                Pacman::update().unwrap_or_else(|report| error!("{}", report));
            }

            // Apt
            Type::Debian | Type::Mint | Type::Pop | Type::Raspbian | Type::Ubuntu => {
                use linux::deb::Deb;

                Deb::update().unwrap_or_else(|report| error!("{}", report));
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

    #[macro_export]
    macro_rules! declare_package_manager_programs {
        (
            $name:ident: [$({
                name: $program_name:ident,
                as_str: $program_name_str:expr,
                commands: [$($command:expr),+],
                is_sudo: $is_sudo:expr $(,)*
            }),+ $(,)*]
            $(,)*
        ) => {
            paste::paste! {
                #[derive(Debug, Copy, Clone, PartialEq, Eq, enum_iterator::Sequence)]
                pub enum [< $name Program >] {
                    $($program_name),*
                }

                impl $crate::PackageManagerProgram for [< $name Program >] {
                    #[inline]
                    fn name(&self) -> &'static str {
                        use [< $name Program >]::*;

                        match self {
                            $(
                                $program_name => $program_name_str
                            ),*
                        }
                    }

                    #[inline]
                    fn update_instructions(&self) -> &[&'static str] {
                        use [< $name Program >]::*;

                        match self {
                            $(
                                $program_name => &[$($command),+]
                            ),*
                        }
                    }

                    fn is_sudo(&self) -> bool {
                        use [< $name Program >]::*;

                        match self {
                            $(
                                $program_name => $is_sudo
                            ),*
                        }
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! declare_package_manager {
        ($name:ident {
            programs: [$({
                name: $program_name:ident,
                as_str: $program_name_str:expr,
                commands: [$($command:expr),+],
                is_sudo: $is_sudo:expr $(,)*
            }),+ $(,)*],
            preferred_program: $preferred_program:ident $(,)*
        }) => {
            paste::paste! {
                declare_package_manager_programs!(
                    $name: [$({
                        name: $program_name,
                        as_str: $program_name_str,
                        commands: [$($command),+],
                        is_sudo: $is_sudo,
                    }),*]
                );

                pub struct $name;

                impl $crate::PackageManager for $name {
                    #[inline]
                    fn get_preferred_program() -> Box<dyn $crate::PackageManagerProgram> {
                        Box::new([< $name Program >]::$preferred_program)
                    }

                    fn get_available_program() -> eyre::Result<Box<dyn $crate::PackageManagerProgram>> {
                        use $crate::PackageManagerProgram;

                        // TODO(tukanoidd): config
                        [< $name Program >]::available_program(Some(
                            [< $name Program >]::$preferred_program,
                        )).map(|program| {
                            let program: Box<dyn PackageManagerProgram> = Box::new(program);
                            program
                        })
                        .map_err(|e| eyre::eyre!("{}", e))
                    }
                }
            }
        };
    }

    pub mod linux {
        pub mod pacman {
            declare_package_manager!(Pacman {
                programs: [
                    {
                        name: Paru,
                        as_str: "paru",
                        commands: ["-Syu"],
                        is_sudo: false,
                    },
                    {
                        name: Yay,
                        as_str: "yay",
                        commands: ["-Syu"],
                        is_sudo: false,
                    },
                    {
                        name: PamacWithAur,
                        as_str: "pamac",
                        commands: ["upgrade", "-a"],
                        is_sudo: false,
                    },
                    {
                        name: Pamac,
                        as_str: "pamac",
                        commands: ["upgrade"],
                        is_sudo: true,
                    },
                    {
                        name: Pacman,
                        as_str: "pacman",
                        commands: ["-Syu"],
                        is_sudo: true,
                    },
                ],
                preferred_program: Paru,
            });
        }

        pub mod deb {
            declare_package_manager!(Deb {
                programs: [
                    {
                        name: Apt,
                        as_str: "apt",
                        commands: ["update"],
                        is_sudo: true,
                    },
                    {
                        name: Aptitude,
                        as_str: "aptitude",
                        commands: ["update"],
                        is_sudo: true,
                    },
                ],
                preferred_program: Aptitude,
            });
        }
    }

    pub mod windows {}

    pub mod macos {}
}

pub mod languages {
    use eyre::*;

    use crate::PackageManager;

    pub fn update() -> Result<()> {
        rust::Rust::update()?;

        Ok(())
    }

    pub mod rust {
        use enum_iterator::Sequence;
        use eyre::*;
        use which::which;

        use crate::{PackageManager, PackageManagerProgram};

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
        pub enum RustProgram {
            Rustup,
            Cargo,
            RustupCargo,
        }

        impl RustProgram {
            pub fn exec_specific_update(self) -> Result<()> {
                let (name, instructions, is_sudo) = match self {
                    RustProgram::Rustup | RustProgram::Cargo => {
                        (self.name(), self.update_instructions(), self.is_sudo())
                    }
                    RustProgram::RustupCargo => bail!("RustupCargo is not a specific update"),
                };

                info!("Starting '{}' update", name);

                if is_sudo {
                    let mut process = runas::Command::new(name).args(instructions).spawn()?;

                    process.wait()?;
                } else {
                    let mut process = std::process::Command::new(name)
                        .args(instructions)
                        .spawn()?;

                    process.wait()?;
                }

                info!("'{}' update is finished", name);

                Ok(())
            }
        }

        impl PackageManagerProgram for RustProgram {
            fn name(&self) -> &'static str {
                match self {
                    RustProgram::Rustup => "rustup",
                    RustProgram::Cargo => "cargo",
                    RustProgram::RustupCargo => "",
                }
            }

            fn update_instructions(&self) -> &[&'static str] {
                match self {
                    RustProgram::Rustup => &["update"],
                    RustProgram::Cargo => &["install-update", "-a"],
                    RustProgram::RustupCargo => &[],
                }
            }

            fn is_sudo(&self) -> bool {
                false
            }

            fn execute_update(&self) -> Result<()> {
                info!("Starting Rust update!");
                let (rustup, cargo) = match self {
                    RustProgram::Rustup => (true, false),
                    RustProgram::Cargo => (false, true),
                    RustProgram::RustupCargo => (true, true),
                };

                if rustup {
                    RustProgram::Rustup.exec_specific_update()?;
                }

                if cargo {
                    RustProgram::Cargo.exec_specific_update()?;
                }

                info!("Rust update is finished");

                Ok(())
            }

            fn is_available(&self) -> bool {
                match self {
                    RustProgram::Rustup | RustProgram::Cargo => which(self.name()).is_ok(),
                    RustProgram::RustupCargo => false,
                }
            }

            fn available_programs() -> Vec<Self>
            where
                Self: Sized + Sequence,
            {
                let mut res = vec![];

                if RustProgram::Rustup.is_available() {
                    res.push(RustProgram::Rustup)
                }

                if RustProgram::Cargo.is_available() {
                    res.push(RustProgram::Cargo)
                }

                res
            }

            fn available_program(preferred_program: Option<Self>) -> Result<Self>
            where
                Self: Sized + Sequence + PartialEq + Copy + Clone,
            {
                let available_programs = Self::available_programs();

                if let Some(preferred_program) = preferred_program {
                    if preferred_program == RustProgram::RustupCargo
                        && available_programs.len() == 2
                    {
                        return Ok(RustProgram::RustupCargo);
                    }
                }

                available_programs
                    .first()
                    .copied()
                    .ok_or(eyre!("No available Rust Program"))
            }
        }

        pub struct Rust;

        impl PackageManager for Rust {
            #[inline]
            fn get_preferred_program() -> Box<dyn PackageManagerProgram> {
                Box::new(RustProgram::RustupCargo)
            }

            #[inline]
            fn get_available_program() -> eyre::Result<Box<dyn PackageManagerProgram>> {
                Err(eyre::eyre!("This should not be called"))
            }

            fn update() -> eyre::Result<()> {
                use RustProgram::*;

                let preferred_program: RustProgram = *Self::get_preferred_program()
                    .downcast_ref()
                    .wrap_err("Failed to downcast RustProgram")?;
                let available_program: RustProgram =
                    RustProgram::available_program(Some(preferred_program))?;

                let (rustup, cargo) = match available_program {
                    Rustup => (true, false),
                    Cargo => (false, true),
                    RustupCargo => (true, true),
                };

                if rustup {
                    Rustup.execute_update()?;
                }

                if cargo {
                    Cargo.execute_update()?;
                }

                Ok(())
            }
        }
    }

    pub mod dart {}

    pub mod python {}

    pub mod go {}
}
