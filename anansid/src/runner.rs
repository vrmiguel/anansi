use std::{
    fmt::Display, matches, process::ExitStatus as StdExitStatus,
    write,
};

use tokio::process::Command;

use crate::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum ExitStatus {
    Code(i32),
    Signaled,
}

impl ExitStatus {
    pub fn from_std(status: StdExitStatus) -> Self {
        status.code().map(Self::Code).unwrap_or(Self::Signaled)
    }

    pub fn was_successful(&self) -> bool {
        matches!(self, Self::Code(0))
    }

    pub fn into_result(self) -> Result<()> {
        if self.was_successful() {
            Ok(())
        } else {
            Err(Error::ProcessFailed(self))
        }
    }
}

impl Display for ExitStatus {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ExitStatus::Code(code) => write!(f, "Code {code}"),
            ExitStatus::Signaled => write!(f, "Signaled"),
        }
    }
}

/// Runs commands asynchronously
pub struct Runner {}

impl Runner {
    pub async fn run_to_completion(command: &str) -> Result<()> {
        let (binary, args) =
            command.split_once(' ').unwrap_or((command, ""));
        // let binary =
        // args.next().ok_or(Error::NoProcessToRun)?;
        let mut child_process =
            Command::new(binary).args([args]).spawn()?;

        let status = {
            let std_status = child_process.wait().await?;

            ExitStatus::from_std(std_status)
        };

        status.into_result()
    }
}
