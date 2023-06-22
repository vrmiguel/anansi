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
