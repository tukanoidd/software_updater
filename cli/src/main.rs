use cfg_if::cfg_if;

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

    software_updater_core::update()
}
