/// Return the magic number
#[inline]
pub fn sol_get_magic_number() -> u64 {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_get_magic_number()
    }

    #[cfg(not(target_os = "solana"))]
    {
        crate::program_stubs::sol_get_magic_number()
    }
}
