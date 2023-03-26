use rand::{distributions::Alphanumeric, Rng};

pub type FolderName = String;
pub type NetworkPort = i16;

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
        rng.gen_range(1000..9999)
    }
}
