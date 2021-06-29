use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    input_node: u32,
    output_node: u32,
    disabled: bool,
    weight: f64,
    innovation_number: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    number: u32,
    node_type: NodeType,
}

#[derive(Serialize, Deserialize)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
}

#[derive(Serialize, Deserialize)]
pub struct Genotype {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Genotype {
    pub fn save(&self, filename_option: Option<String>) -> Result<(), GenotypeError> {
        let filename: String;
        match filename_option {
            Some(file) => filename = file,
            _ => filename = "network.json".to_string(),
        }
        let file = match OpenOptions::new().write(true).create(true).truncate(true).open(filename) {
            Ok(filestream) => filestream,
            _ => return Err(GenotypeError::FileSystemError),
        };
        match serde_json::to_writer(&file, &self) {
            Ok(_) => return Ok(()),
            _ => return Err(GenotypeError::SerializingError),
        };
    }

    pub fn load(filename_option: Option<String>) -> Result<Genotype, GenotypeError> {
        let filename: String;
        match filename_option {
            Some(file) => filename = file,
            _ => filename = "network.json".to_string(),
        }
        let file = match OpenOptions::new().read(true).open(filename) {
            Ok(filestream) => filestream,
            _ => return Err(GenotypeError::FileNotFound),
        };
        match serde_json::from_reader(file) {
            Ok(data) => return Ok(data),
            _ => return Err(GenotypeError::DeserializingError),
        };
    }

    pub fn new(num_of_input: u32, num_of_output: u32) -> Genotype {
        let mut nodes: Vec<Node> = vec![];
        for i in 0..num_of_input + 1 {
            nodes.push(Node {
                number: i,
                node_type: NodeType::Input,
            });
        }
        for i in num_of_input + 1..num_of_output + num_of_input + 1{
            nodes.push(Node {
                number: i,
                node_type: NodeType::Output,
            });
        }

        let mut links: Vec<Link> = vec![];
        for i in 0..num_of_input + 1 {
            for j in num_of_input + 1..num_of_output + num_of_input + 1{
                links.push(Link {
                    disabled: false,
                    innovation_number: 0,
                    weight: 1.0,
                    input_node: i,
                    output_node: j,
                });
            }
        }
        return Genotype {
            nodes: nodes,
            links: links,
        };
    }
}

pub enum GenotypeError {
    FileNotFound,
    FileSystemError,
    SerializingError,
    DeserializingError,
}

impl std::fmt::Display for GenotypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There has been an error writing or reading the genotype")
    }
}
impl std::fmt::Debug for GenotypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There has been an error writing or reading the genotype")
    }
}
