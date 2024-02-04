mod user;

use user::{
    User,
};
use crate::user::Sex;

fn main() {
    let mut user = User::new(
        String::from("user1"),
        String::from("user1@email.com"),
        Sex::Man,
    );
    println!("user is {:?}", user);
    user.login();
    println!("user's sign_in_count is {}", user.get_sign_in_count());

    user.disable();
    println!("new user is {:?}", user);

    user.show_sex()
}
