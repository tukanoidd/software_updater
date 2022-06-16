#[macro_use]
extern crate log;

pub mod os;

use std::{collections::HashMap, hash::Hash, io::Write, path::PathBuf, process::Command};

use enum_iterator::{all, Sequence};
use eyre::*;
use which::which;

pub trait UpdateRoutine<P: Available + PackageManager>
where
    &'static str: From<P>,
{
    fn update(&self) -> Result<()> {
        info!("Updating {} Packages", P::up_case_name());

        let (package_manager, path) = P::available(Self::preferred_package_manager())?;
        let result = Command::new(path)
            .arg(package_manager.update_instruction())
            .spawn()?
            .wait_with_output()?;

        std::io::stdout()
            .lock()
            .write_all(result.stdout.as_slice())?;
        debug!("Exit Status: {:?}", result.status);

        info!("Finished updating {} Packages", P::up_case_name());

        Ok(())
    }

    fn preferred_package_manager() -> Option<P>;
}

pub trait Available: Sequence
where
    &'static str: From<Self>,
    Self: Sized + PartialEq + Eq + Copy + Clone + Hash,
{
    fn available_name() -> &'static str;

    #[inline]
    fn available(preferred: Option<Self>) -> Result<(Self, PathBuf)> {
        let available_map = Self::available_map();

        if let Some(preferred) = preferred {
            if let Some(path) = available_map.get(&preferred) {
                return Ok((preferred, path.clone()));
            }
        }

        return available_map
            .iter()
            .next()
            .map(|(s, path)| (*s, path.clone()))
            .ok_or_else(|| eyre!("No available {} found", Self::available_name()));
    }

    #[inline]
    fn available_map() -> HashMap<Self, PathBuf> {
        HashMap::from_iter(all::<Self>().filter_map(|s| {
            if let Some(path) = s.is_available() {
                Some((s, path))
            } else {
                None
            }
        }))
    }

    #[inline]
    fn is_available(&self) -> Option<PathBuf> {
        which::<&str>((*self).into()).ok()
    }
}

pub trait PackageManager
where
    &'static str: From<Self>,
    Self: Sized + Copy,
{
    fn update_instruction(&self) -> &'static str;
    fn up_case_name() -> &'static str;
}
