use anyhow::Context;

const IPC_PATH: &str = "/dev/shm/simplekanainput.ipc.dat";

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum IpcState {
    Visible,
    Hidden,
    ShowRequested,
    QuitRequested,
}

impl IpcState {
    #[cfg(feature = "ipc")]
    pub fn read() -> anyhow::Result<Self> {
        let data = std::fs::read(IPC_PATH)?;
        Self::from_byte(data[0]).context("Invalid state value")
    }

    #[cfg(not(feature = "ipc"))]
    pub fn read() -> anyhow::Result<Self> {
        anyhow::bail!("ipc not enabled on web")
    }

    #[cfg(feature = "ipc")]
    pub fn write(self) -> std::io::Result<()> {
        std::fs::write(IPC_PATH, [self as u8])
    }

    #[cfg(not(feature = "ipc"))]
    pub fn write(self) -> std::io::Result<()> {
        Ok(())
    }

    #[cfg(feature = "ipc")]
    fn from_byte(byte: u8) -> Option<Self> {
        Some(match byte {
            0 => Self::Visible,
            1 => Self::Hidden,
            2 => Self::ShowRequested,
            3 => Self::QuitRequested,
            _ => return None,
        })
    }

    #[cfg(feature = "ipc")]
    pub fn remove() -> std::io::Result<()> {
        std::fs::remove_file(IPC_PATH)
    }

    #[cfg(not(feature = "ipc"))]
    pub fn remove() -> std::io::Result<()> {
        Ok(())
    }
}
