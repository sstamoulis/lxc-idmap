mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn geteuid() -> u32;
    }
}

pub fn geteuid() -> u32 {
    unsafe { libc::geteuid() }
}
