use crate::message::Message;

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error<'a> {
    InvalidTag { invalid_tag: &'a str },
    InvalidChannel(&'a str),
    InvalidSend(&'a str),
}

// TODO: accept &[u8]?
pub fn parse(input: &'_ str) -> Result<Message<'_>> {
    // Ensure the ending is trimmed
    let input = input.trim_end();

    assert!(input.len() >= 2);
    let (tag, body) = input.split_at(2);

    match tag {
        "SN" => {
            // Send and event to the corresponding channel
            if body.contains(' ') {
                return Err(Error::InvalidChannel(body));
            }

            Ok(Message::Send { channel_name: body })
        }
        "ON" => {
            // Wait for an event

            let (channel_name, command) =
                body.split_once(' ')
                    .ok_or(Error::InvalidSend(body))?;

            Ok(Message::On {
                channel_name,
                run: command,
            })
        }
        "CL" => Ok(Message::Close),
        _ => return Err(Error::InvalidTag { invalid_tag: tag }),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::parse;
    use crate::decoder::{Error, Message};

    #[test]
    fn parses_commands_correctly() {
        assert_eq!(
            parse("SNclean_cache"),
            Ok(Message::Send {
                channel_name: "clean_cache"
            })
        );

        assert_eq!(
            parse("ONclean_tmp rm -rf /tmp"),
            Ok(Message::On {
                channel_name: "clean_tmp",
                run: "rm -rf /tmp"
            })
        );

        assert_eq!(parse("CL"), Ok(Message::Close));

        assert_eq!(
            parse("SENDreboot"),
            Err(Error::InvalidTag { invalid_tag: "SE" })
        );
        assert_eq!(
            parse("ONclean_tmp"),
            Err(Error::InvalidSend("clean_tmp"))
        );
    }
}
