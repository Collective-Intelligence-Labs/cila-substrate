use crate::command::{Command};
use sp_core::{H160};


pub trait CommandUtils  {
    fn get_aggregate_id_h160(&self) -> Result<H160, &str>;
}

impl CommandUtils for Command {
    fn get_aggregate_id_h160(&self) -> Result<H160, &str> {
        if self.aggregate_id.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.aggregate_id);
            Ok(buf.into())
        } else {
            Err("Aggregate ID must be 20-bytes in length")
        }
    }
}