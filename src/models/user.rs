
#[derive(Debug, PartialEq, Eq)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_active: bool,
    pub is_staff: bool,
}