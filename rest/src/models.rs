use crate::entities;

pub fn fetch_users() -> Vec<entities::User> {
    vec![
        entities::User {
            id: 1,
            name: String::from("John"),
            email: String::from("john@example.com"),
        },
        entities::User {
            id: 1,
            name: String::from("James"),
            email: String::from("james@example.com"),
        },
    ]
}
