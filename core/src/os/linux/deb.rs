use enum_iterator::Sequence;

use crate::{Available, PackageManager, UpdateRoutine};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum DebPackageManager {
    Apt,
    Aptitude,
}

impl Available for DebPackageManager {
    #[inline]
    fn available_name() -> &'static str {
        "apt package manager"
    }
}

impl PackageManager for DebPackageManager {
    fn update_instruction(&self) -> &'static str {
        "update"
    }

    fn up_case_name() -> &'static str {
        "Apt"
    }
}

impl From<DebPackageManager> for &'static str {
    fn from(rhs: DebPackageManager) -> Self {
        match rhs {
            DebPackageManager::Apt => "apt",
            DebPackageManager::Aptitude => "aptitude",
        }
    }
}

pub struct Deb;

impl UpdateRoutine<DebPackageManager> for Deb {
    fn preferred_package_manager() -> Option<DebPackageManager> {
        //TODO(tukanoidd): config
        Some(DebPackageManager::Apt)
    }
}
