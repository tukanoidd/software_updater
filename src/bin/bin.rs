use cfg_if::cfg_if;

use software_updater_core::{
    os::linux::arch::{ArchBuilder, AurPackageManager},
    UpdateRoutine,
};

fn main() {
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
        .aur(false)
        .preferred_aur_package_manager(Some(AurPackageManager::Paru))
        .build()
        .expect("Couldn't build Arch config");

    arch.update().expect("Couldn't update packages");
}
