use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("MBR is  being used incorrectly")]
    Incompatible,
    #[error("unknown cmd error")]
    Unknown,
}

// impl ErrorCodeExt for Error {
//     fn error_code(&self) -> ErrorCode {
//         match self {
//             Error::Incompatible => error_code::UNKNOWN,
//         }
//     }
// }
