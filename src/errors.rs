use std::fmt;
use std::process::exit;

#[derive(Debug)]
// Contains all possible errors in the CLI tool
pub enum Errcode {
    ContainerError(u8),
    NotSupported(u8),
    ArgumentInvalid(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum to be displayed by:
//      println!("{}", error);
//  in this case, it calls the function "fmt", which we define the behaviour below
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ContainerError(element) => write!(f, "ContainerError: {}", element),
            Errcode::NotSupported(element) => write!(f, "NotSupported: {}", element),
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),

            _ => write!(f, "{:?}", self),
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // Sucess -> return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }
        // If there's an error, print the message and return the code
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit: \n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}

impl Errcode {
    /// Translate an `Errcode::x` into a number to return (the Unix way)
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}
