use crate::constants;

use keyring;

pub fn get_password(username: &str) -> Result<String, keyring::Error> {
    let entry = keyring::Entry::new(constants::APPLICATION_NAME, &username);

    entry.get_password()
}

pub fn set_password(username: &str, password: &str) {
    let entry = keyring::Entry::new(constants::APPLICATION_NAME, &username);

    entry.set_password(&password).unwrap();
}
