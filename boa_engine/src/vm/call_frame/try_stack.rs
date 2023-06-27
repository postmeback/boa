// TODO(HalidOdat): remove this
#![allow(dead_code)]

/// The `EnvStackEntry` tracks the environment count and relevant information for the current environment.
#[derive(Clone, Debug)]
pub(crate) struct TryStackEntry {
    catch: u32,

    /// The length of the value stack when the try block was entered.
    ///
    /// This is used to pop exact amount values from the stack
    /// when a throw happens.
    fp: u32,

    env_fp: u32,
}

impl TryStackEntry {
    /// Creates a new [`EnvStackEntry`] with the supplied start addresses.
    pub(crate) const fn new(catch: u32, fp: u32, env_fp: u32) -> Self {
        Self { catch, fp, env_fp }
    }

    pub(crate) const fn catch(&self) -> u32 {
        self.catch
    }

    pub(crate) const fn fp(&self) -> u32 {
        self.fp
    }

    pub(crate) const fn env_fp(&self) -> u32 {
        self.env_fp
    }
}
