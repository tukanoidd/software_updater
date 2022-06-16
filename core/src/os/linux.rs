pub mod arch;
pub mod deb;

use eyre::*;

/*use crate::os::linux::pacman::AurPackageManager;*/
use crate::UpdateRoutine;

use self::{arch::Arch, deb::Deb};

pub enum LinuxPackageManager {
    Arch(Arch),
    Deb(Deb),
    None,
}

impl LinuxPackageManager {
    pub fn update(&self) -> Result<()> {
        match self {
            LinuxPackageManager::Arch(pacman) => pacman.update()?,
            LinuxPackageManager::Deb(deb) => deb.update()?,
            LinuxPackageManager::None => error!("No supported linux package manager"),
        }

        Ok(())
    }
}
