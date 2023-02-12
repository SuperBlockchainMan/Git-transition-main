pub use transition_macros::{versioned, versioned_enum, impl_version};

pub trait Versioned {
    const VERSION: u64;
    const VERSION_VARINT_SIZE_BYTES: usize;
}