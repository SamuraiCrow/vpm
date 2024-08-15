mod cmd;
mod install;
mod uninstall;
// mod init;
// mod docs;
mod dotf;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    fn execute(&self) -> Result<()>;
}

impl Execute for Cmd {
    fn execute(&self) -> Result<()> {
        match self {
            Cmd::Install(cmd) => cmd.execute(),
            Cmd::Uninstall(cmd) => cmd.execute(),
            // Cmd::Init(cmd) => cmd.execute(),
            // Cmd::Docs(cmd) => cmd.execute(),
            Cmd::Dotf(cmd) => cmd.execute(),
        }
    }
}
