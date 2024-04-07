use crate::config::ContainerOpts;
use crate::errors::Errcode;
use crate::hostname::set_contianer_hostname;
use crate::mounts::set_container_mountpoint;

use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::Pid;

const STACK_SIZE: usize = 1024 * 1024;

fn child(config: ContainerOpts) -> isize {
    match setup_container_configurations(&config) {
        Ok(_) => log::info!("Container set up successfully"),
        Err(e) => {
            log::error!("Error while configuring crabcan: {:?}", e);
            return -1;
        }
    }
    log::info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    0
}

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);

    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Errcode::ChildProcessError(0)),
    }
}

fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
    set_contianer_hostname(&config.hostname)?;
    set_container_mountpoint(&config.mount_dir)?;
    Ok(())
}
