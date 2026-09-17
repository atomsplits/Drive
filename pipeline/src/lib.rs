use std::io::Write;

use ipckit::{IpcError, NamedPipe};

fn connect_windows() -> Result<(), IpcError> {
    for i in 0..9 {
        let mut pipe = NamedPipe::connect(format!(r"\\.pipe\discord-ipc-{}", i).as_mut_str())?;
        pipe.wait_for_client()?;
    }

    Ok(())
}
