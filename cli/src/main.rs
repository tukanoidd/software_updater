use cfg_if::cfg_if;

use eyre::*;
use software_updater_core::{
    os::linux::{arch::ArchBuilder, deb::Deb},
    UpdateRoutine,
};

fn main() -> Result<()> {
    cfg_if! {
        if #[cfg(debug_assetions)] {
            std::env::set_var("RUST_LOG", "debug");
        } else if #[cfg(feature = "log_trace")] {
            std::env::set_var("RUST_LOG", "trace");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    pretty_env_logger::init_timed();

    let arch = ArchBuilder::default()
        .pacman(true)
        .aur(true)
        .build()
        .expect("Couldn't build Arch config");

    arch.update()

    /*(Deb {}).update()*/
}
