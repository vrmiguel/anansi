#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message<'a> {
    /// Tag: "SN"
    Send { channel_name: &'a str },
    /// Tag: "ON"
    On { channel_name: &'a str, run: &'a str },
    /// Tag: "CL"
    // TODO: remove the Close command?
    Close,
}

pub enum Response {
    Ok,
    ChannelNotFound,
    InternalError,
    ProcessFailed,
    Unimplemented,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Response::Ok => b"OK",
            Response::ChannelNotFound => b"NOT_FOUND",
            Response::InternalError => b"INTERNAL_ERR",
            Response::ProcessFailed => b"PROC_FAILED",
            Response::Unimplemented => b"UNIMPLEMENTED",
        }
    }
}
