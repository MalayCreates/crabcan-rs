use std::os::fd::RawFd;

use nix::{
    sched::{unshare, CloneFlags},
    unistd::Pid,
};

use crate::{
    errors::Errcode,
    ipc::{recv_boolean, send_boolean},
};

pub fn userns(fd: RawFd, uid: u32) -> Result<(), Errcode> {
    log::debug!("Setting up user namespace with UID {}", uid);
    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_boolean(fd, has_userns)?;
    if recv_boolean(fd)? {
        return Err(Errcode::NamespaceError(0));
    }
    if has_userns {
        log::info!("User namespaces set up");
    } else {
        log::info!("User namespaces not supported, continuing...");
    }
    Ok(())
}

pub fn handle_child_uid_map(pid: Pid, fd: RawFd) -> Result<(), Errcode> {
    if recv_boolean(fd)? {
        // perform mapping
    } else {
        log::info!("No user namespace set up from child process");
    }
    log::debug!("Child UID/GID map done, sending signal to child to continue...");
    send_boolean(fd, false)
}
