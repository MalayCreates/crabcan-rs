use std::fmt;
use std::process::exit;

// Allows to display variant of format {:?}
#[derive(Debug)]
// Contains all known errors in tool
pub enum Errcode {
    ArgumentInvalid(&'static str),
}
#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum too be displayed by:
//      println!("{}", error);
// in this case, it calls the function "fmt", which we define the behavior below
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behavior for each variant of the enum
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}

// Get the result from a function, and exit the process wtih the correct error
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If success, return 0
        Ok(_) => {
            log::debug!("Exit without an error, returning 0");
            exit(0);
        }
        // If error, print error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}

impl Errcode {
    // Translate an Errcode::X into a number to return (the unix way :O)
    pub fn get_retcode(&self) -> i32 {
        1 // Everything ! = 0 will be treated as an error
    }
}
