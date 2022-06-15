use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux::Linux;
    }
}

pub struct Os {
    #[cfg(target_os = "linux")]
    pub linux: Linux,
}
