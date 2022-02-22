use crate::constants;

use keyring;

pub fn get_password(username: &str) -> Result<String, keyring::Error> {
    let service = constants::APPLICATION_NAME;

    let entry = keyring::Entry::new(&service, &username);

    entry.get_password()
}
