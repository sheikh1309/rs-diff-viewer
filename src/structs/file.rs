use crate::enums::change_type::ChangeType;

#[derive(Debug, Clone)]
pub struct File {
    pub change_type: ChangeType,
    pub title: String,
    pub meta: String,
    pub icon: String
}