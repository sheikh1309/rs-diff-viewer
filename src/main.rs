use crate::structs::file_tree::FileTree;

mod structs;
mod enums;

pub fn main() -> iced::Result {
    iced::application("My app", FileTree::update, FileTree::view)
        .window_size(iced::Size::new(1600.0, 1200.0))
        .run_with(|| (FileTree::new(), iced::Task::none()))
}
