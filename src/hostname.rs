use nix::unistd::sethostname;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::errors::Errcode;

const HOSTNAME_NAMES: [&'static str; 8] = [
    "poetry", "mood", "fortune", "menu", "customer", "wolf", "cabinet", "leopard",
];
const HOSTNAME_ADJ: [&'static str; 16] = [
    "macho",
    "salty",
    "defiant",
    "strong",
    "same",
    "shallow",
    "late",
    "victorious",
    "boiling",
    "sad",
    "mad",
    "cheerful",
    "deep",
    "brutal",
    "quick",
    "anxious",
];

pub fn generate_hostname() -> Result<String, Errcode> {
    let mut r = rand::thread_rng();
    let num = r.gen::<u8>();
    let name = HOSTNAME_NAMES.choose(&mut r).ok_or(Errcode::RNGError)?;
    let adj = HOSTNAME_ADJ.choose(&mut r).ok_or(Errcode::RNGError)?;
    Ok(format!("{}-{}-{}", adj, name, num))
}

pub fn set_contianer_hostname(hostname: &String) -> Result<(), Errcode> {
    match sethostname(hostname) {
        Ok(_) => {
            log::debug!("Container hostname is now {}", hostname);
            Ok(())
        }
        Err(_) => {
            log::error!("Cannot set hostname {} for container", hostname);
            Err(Errcode::HostnameError(0))
        }
    }
}
