pub enum Error {
    FILE_NOT_FOUND(String),
}

pub enum READ_RESULT<T> {
    TRUE(T),
    FALSE,
    ERROR,
}
impl<T> READ_RESULT<T> {
    pub fn get_context(&self) -> &T {
        match self {
            READ_RESULT::TRUE(context) => context.to_owned(),
            READ_RESULT::FALSE => todo!(),
            READ_RESULT::ERROR => todo!(),
        }
    }
}
