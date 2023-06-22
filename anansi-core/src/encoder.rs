use std::{fmt::Display, write};

use crate::Message;

impl Display for Message<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Message::Send { channel_name } => {
                write!(f, "SN{channel_name}")
            }
            Message::On { channel_name, run } => {
                write!(f, "ON{channel_name} {run}")
            }
            Message::Close => f.write_str("CL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Message;

    #[test]
    fn encodes_messages_correctly() {
        assert_eq!(
            Message::On {
                channel_name: "clean_tmp",
                run: "rm -rf /tmp",
            }
            .to_string(),
            "ONclean_tmp rm -rf /tmp"
        );

        assert_eq!(
            Message::Send {
                channel_name: "clean_tmp",
            }
            .to_string(),
            "SNclean_tmp"
        );

        assert_eq!(Message::Close.to_string(), "CL");
    }
}
