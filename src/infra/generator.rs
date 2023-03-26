use rand::{distributions::Alphanumeric, Rng};

pub type FolderName = String;
pub type NetworkPort = u16;

#[derive(Clone, Default)]
pub struct Generator;

pub trait Randomize {
    fn generate_folder_name(&self) -> FolderName;
    fn generate_port(&self) -> NetworkPort;
}

impl Randomize for Generator {
    fn generate_folder_name(&self) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::{FolderName, NetworkPort, Randomize};

    pub struct MockGenerator {
        folder_values: Vec<FolderName>,
        port_values: Vec<NetworkPort>,
    }
    impl MockGenerator {
        pub fn new(folder_values: Vec<FolderName>, port_values: Vec<NetworkPort>) -> Self {
            MockGenerator { folder_values, port_values }
        }
    }
    impl Randomize for MockGenerator {
        fn generate_folder_name(&self) -> FolderName {
            self.folder_values.first().unwrap().to_string()
        }

        fn generate_port(&self) -> NetworkPort {
            self.port_values.first().unwrap().clone()
        }
    }
}
