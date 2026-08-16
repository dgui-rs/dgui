#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid root widget: the root should always be MainPanel. use : Widget::main().")]
    InvalidRootWidget,
}

pub type Result<T> = std::result::Result<T, Error>;
