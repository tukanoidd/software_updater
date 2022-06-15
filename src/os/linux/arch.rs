use std::io::Write;
use std::process::{Command, Stdio};

use derive_builder::Builder;
use enum_iterator::Sequence;
use eyre::*;
use getset::{Getters, Setters};
use which::which;

use crate::{Available, PackageManager, UpdateRoutine};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
pub enum AurPackageManager {
    Yay,
    AurUtils,
    Pikaur,
    Paru,
    Pamac,
}

impl AurPackageManager {}

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
    #[getset(get)]
    pacman: bool,

    #[getset(get)]
    aur: bool,

    #[getset(get)]
    preferred_aur_package_manager: Option<AurPackageManager>,
}

impl Arch {
    pub fn update_pacman() -> Result<()> {
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

    pub fn update_aur(preferred: Option<AurPackageManager>) -> Result<()> {
        info!("Updating Aur Packages");

        let aur_package_manager = AurPackageManager::available(preferred)?;
        let result = Command::new::<&str>(aur_package_manager.into())
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
            preferred_aur_package_manager: None,
        }
    }
}

impl UpdateRoutine for Arch {
    fn update(&self) -> Result<()> {
        let mut updated = false;

        if self.pacman {
            Self::update_pacman()?;
            updated = true;
        }

        if self.aur {
            Self::update_aur(self.preferred_aur_package_manager)?;
            updated = true;
        }

        if updated {
            info!("Arch/Aur packages updated!")
        }

        Ok(())
    }
}
