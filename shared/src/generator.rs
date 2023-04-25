use passwords::PasswordGenerator;
use rand::{distributions::Alphanumeric, Rng};

pub type FolderName = String;
pub type ContainerName = String;
pub type NetworkPort = u16;
pub type Password = String;
pub type Username = String;

#[derive(Clone, Default)]
pub struct DefaultGenerator;

pub trait Generator {
    fn generate_folder_name(&self) -> FolderName;
    fn generate_port(&self) -> NetworkPort;
    fn generate_username(&self) -> Username;
    fn generate_password(&self, length: usize) -> Password;
    fn generate_container_name(&self) -> ContainerName;
}

impl Generator for DefaultGenerator {
    fn generate_folder_name(&self) -> FolderName {
        const STRING_LENGTH: usize = 14;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(STRING_LENGTH)
            .map(char::from)
            .collect()
    }

    fn generate_port(&self) -> NetworkPort {
        let mut rng = rand::thread_rng();
        rng.gen_range(1000..=65535)
    }

    fn generate_username(&self) -> Username {
        const USERNAME_LENGTH: usize = 8;
        let adjectives = [
            "mighty", "brave", "strong", "fast", "smart", "great", "fierce", "bold",
            "creative", "happy", "calm",
        ];
        let mut rng = rand::thread_rng();

        let adjective = adjectives
            .get(rng.gen_range(0..adjectives.len()))
            .unwrap_or(&"mighty");
        let random_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(USERNAME_LENGTH)
            .map(char::from)
            .collect();

        format!("{}-{}", adjective, random_string)
    }

    fn generate_password(&self, length: usize) -> Password {
        let pg = PasswordGenerator {
            length,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        pg.generate_one()
            .expect("configuration is able to generate a passwords")
    }

    fn generate_container_name(&self) -> ContainerName {
        const STRING_LENGTH: usize = 14;
        let tmp: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(STRING_LENGTH)
            .map(char::from)
            .collect();
        tmp.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::{FolderName, Generator, NetworkPort};

    pub struct MockGenerator {
        folder_values: Vec<FolderName>,
        port_values: Vec<NetworkPort>,
    }
    impl MockGenerator {
        pub fn new(
            folder_values: Vec<FolderName>,
            port_values: Vec<NetworkPort>,
        ) -> Self {
            MockGenerator { folder_values, port_values }
        }
    }
    impl Generator for MockGenerator {
        fn generate_folder_name(&self) -> FolderName {
            self.folder_values.first().unwrap().to_string()
        }

        fn generate_port(&self) -> NetworkPort {
            self.port_values.first().unwrap().clone()
        }

        fn generate_username(&self) -> super::Username {
            String::from("mighty-user")
        }

        fn generate_password(&self, _length: usize) -> super::Password {
            String::from("mighty-passWord-4")
        }

        fn generate_container_name(&self) -> super::ContainerName {
            todo!()
        }
    }
}
