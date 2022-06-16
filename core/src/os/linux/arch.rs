use std::{io::Write, process::Command};

use derive_builder::Builder;
use enum_iterator::Sequence;
use eyre::*;
use getset::{Getters, Setters};
use which::which;

use crate::{Available, PackageManager, UpdateRoutine};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum AurPackageManager {
    Yay,
    AurUtils,
    Pikaur,
    Paru,
    Pamac,
}

impl Available for AurPackageManager {
    fn available_name() -> &'static str {
        "aur package manager"
    }
}

impl PackageManager for AurPackageManager {
    fn update_instruction(&self) -> &'static str {
        match self {
            AurPackageManager::Yay => "-Syu --aur",
            AurPackageManager::AurUtils => "sync -u",
            AurPackageManager::Pikaur | AurPackageManager::Paru => "-Sua",
            AurPackageManager::Pamac => "upgrade",
        }
    }

    fn up_case_name() -> &'static str {
        "Aur"
    }
}

impl UpdateRoutine<AurPackageManager> for AurPackageManager {
    fn preferred_package_manager() -> Option<AurPackageManager> {
        //TODO(tukanoidd): config
        Some(AurPackageManager::Paru)
    }
}

impl From<AurPackageManager> for &'static str {
    fn from(rhs: AurPackageManager) -> Self {
        match rhs {
            AurPackageManager::Yay => "yay",
            AurPackageManager::AurUtils => "aur",
            AurPackageManager::Pikaur => "pikaur",
            AurPackageManager::Paru => "paru",
            AurPackageManager::Pamac => "pamac",
        }
    }
}

#[derive(Getters, Setters, Builder)]
pub struct Arch {
    #[getset(get_copy = "pub")]
    pacman: bool,

    #[getset(get_copy = "pub")]
    aur: bool,
}

impl Arch {
    pub fn update_pacman(&self) -> Result<()> {
        info!("Updating Arch Packages");

        let sudo = which("sudo")?;
        let pacman = which("pacman")?;
        let result = Command::new(sudo)
            .arg("-S")
            .arg(pacman)
            .arg("-Syu")
            .spawn()?
            .wait_with_output()?;

        std::io::stdout().write_all(result.stdout.as_slice())?;
        debug!("Exit Status: {:?}", result.status);

        Ok(())
    }

    pub fn update_aur(&self) -> Result<()> {
        info!("Updating Aur Packages");

        let (aur_package_manager, path) =
            AurPackageManager::available(Self::preferred_package_manager())?;
        let result = Command::new(path)
            .arg(aur_package_manager.update_instruction())
            .spawn()?
            .wait_with_output()?;

        std::io::stdout()
            .lock()
            .write_all(result.stdout.as_slice())?;
        debug!("Exit Status: {:?}", result.status);

        Ok(())
    }
}

impl Default for Arch {
    #[inline]
    fn default() -> Self {
        Self {
            pacman: true,
            aur: true,
        }
    }
}

impl UpdateRoutine<AurPackageManager> for Arch {
    fn update(&self) -> Result<()> {
        let mut updated = false;

        if self.pacman {
            self.update_pacman()?;
            updated = true;
        }

        if self.aur {
            self.update_aur()?;
            updated = true;
        }

        if updated {
            info!("Arch/Aur packages updated!")
        }

        Ok(())
    }

    #[inline]
    fn preferred_package_manager() -> Option<AurPackageManager> {
        AurPackageManager::preferred_package_manager()
    }
}
