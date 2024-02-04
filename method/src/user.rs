#[allow(dead_code)]
#[derive(Debug)]
pub enum Sex {
    Man,
    Woman,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    sex: Sex,
}

// impl block is used to define methods for a struct
// It can be multiple impl blocks for a struct.
impl User {
    // associated function, e.g. String::from
    // Use `::` to call it, e.g. User::new
    pub fn new(username: String, email: String, sex: Sex) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
            sex,
        }
    }
    // 1. &self is a reference to the instance of User, equal to self: &User
    // 2. &mut self is a mutable reference to the instance of User, equal to self: &mut User
    // 3. self will move ownership.
    pub fn login(&mut self) {
        self.sign_in_count += 1;
    }
    pub fn disable(&mut self) {
        self.active = false;
    }
    pub fn get_sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
    pub fn show_sex(&self) {
        self.sex.tips()
    }
}

// enum can also have methods.
impl Sex {
    pub fn tips(&self) {
        println!("field is {:?}", self);
    }
}