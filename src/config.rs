use crate::errors::Errcode;
use crate::ipc::generate_socketpair;

use crate::hostname::{self, generate_hostname};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;
#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,

    pub fd: RawFd,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub hostname: String,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].clone();

        Ok((
            ContainerOpts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
                hostname: generate_hostname()?,
            },
            sockets,
        ))
    }
}
