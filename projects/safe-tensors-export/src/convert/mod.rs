use crate::Application;

impl Application {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("it works!");
        Ok(())
    }
}