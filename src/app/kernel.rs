use crate::FilacoResult;

pub struct Kernel {}

impl Kernel {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn init(&mut self) -> FilacoResult<()> {
        Ok(())
    }

    pub async fn run(&self) -> FilacoResult<()> {
        Ok(())
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}