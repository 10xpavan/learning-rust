mod frontofhouse {
    pub mod hosting{
        pub fn addtowaitlist() {}

    }
}

pub fn eatatrestaurant() {
    crate::frontofhouse::hosting::addtowaitlist();

    frontofhouse::hosting::addtowaitlist();
}