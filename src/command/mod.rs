mod build;
mod entry;

pub use self::build::Build;
pub use self::entry::Entry;

pub trait Command {
    fn run(&mut self) -> Result<(), ()>;
}
