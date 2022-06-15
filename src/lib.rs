#[macro_use]
extern crate log;

pub mod os;

use enum_iterator::{all, Sequence};
use eyre::*;
use which::which;

pub trait UpdateRoutine {
    fn update(&self) -> Result<()>;
}

pub trait Available: Sequence
where
    &'static str: From<Self>,
    Self: Sized + PartialEq + Copy + Clone,
{
    fn available_name() -> &'static str;

    #[inline]
    fn available(preferred: Option<Self>) -> Result<Self> {
        let available_list = Self::available_list();

        if let Some(preferred) = preferred {
            if available_list.contains(&preferred) {
                return Ok(preferred);
            }
        }

        return available_list
            .first()
            .cloned()
            .ok_or_else(|| eyre!("No available {} found", Self::available_name()));
    }

    #[inline]
    fn available_list() -> Vec<Self> {
        all::<Self>().filter(|shell| shell.is_available()).collect()
    }

    #[inline]
    fn is_available(&self) -> bool {
        which::<&str>((*self).into()).is_ok()
    }
}

pub trait PackageManager
where
    &'static str: From<Self>,
    Self: Sized + Copy,
{
    fn update_instruction(&self) -> &'static str;
}
