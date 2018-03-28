mod build;
mod each;
mod entry;

pub use self::build::Build;
pub use self::each::Each;
pub use self::entry::Entry;

pub trait Command {
    fn run(&mut self) -> Result<(), ()>;
}
