#[derive(Fail, Debug)]
pub enum LinkedListError {
    #[fail(display = "Index out of bounds")]
    IndexOutOfRangeError,
}

pub type Result<T> = std::result::Result<T, LinkedListError>;
