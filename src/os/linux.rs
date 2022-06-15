pub mod arch;

use eyre::*;

use crate::UpdateRoutine;

use self::arch::Arch;

pub enum Linux {
    Arch(Arch),
    None,
}

impl UpdateRoutine for Linux {
    fn update(&self) -> Result<()> {
        match self {
            Linux::Arch(arch) => arch.update(),
            Linux::None => bail!("No supported system was found"),
        }
    }
}
