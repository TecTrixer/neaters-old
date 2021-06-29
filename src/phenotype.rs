use crate::genotype::*;

#[derive(Clone)]
pub struct Phenotype {
    nodes: Vec<PhenoNodes>,
}

#[derive(Clone, Debug)]
pub struct PhenoNodes {
    number: u32,
    node_type: NodeType,
    links: Vec<PhenoLinks>,
    solved: bool,
    value: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct PhenoLinks {
    source_node: u32,
    weight: f64,
}

impl Phenotype {
    pub fn feed(&mut self, mut inputs: Vec<f64>) -> Vec<f64> {
        inputs.insert(0, 1.0);
        let mut outputs: Vec<f64> = vec![];
        for output_node in self.clone()
            .nodes
            .iter()
            .filter(|node| node.node_type == NodeType::Output)
        {
            outputs.push(activation(self.recursive_feed(&mut output_node.clone(), &inputs)))
        }
        return outputs;
    }

    fn recursive_feed(&mut self, mut node: &mut PhenoNodes, inputs: &Vec<f64>) -> f64 {
        for link in &node.links {
            let mut source_node: PhenoNodes = self.clone()
                .nodes
                .into_iter()
                .find(|node| node.number == link.source_node)
                .unwrap();
            if source_node.solved {
                node.value += activation(source_node.value) * link.weight;
            }
            else if source_node.node_type == NodeType::Input {
                node.value += activation(inputs[source_node.number as usize]) * link.weight;
            } else {
                node.value += activation(self.recursive_feed(&mut source_node, inputs)) * link.weight;
            }
        }
        node.solved = true;
        return node.value;
    }

    pub fn from_genotype(mut genotype: Genotype) -> Phenotype {
        let mut nodes: Vec<PhenoNodes> = vec![];
        for node in genotype.nodes {
            nodes.push(PhenoNodes {node_type: node.node_type, number: node.number, solved: false, value: 0.0, links: vec![]})
        }
        for link in genotype.links.iter().filter(|link| !link.disabled) {
            let node = nodes.iter_mut().find(|node| node.number == link.output_node).unwrap();
            node.links.push(PhenoLinks {source_node: link.input_node, weight: link.weight});
        }
        println!("{:?}", nodes);
        return Phenotype {nodes}
    }
}

fn activation(value: f64) -> f64 {
    return 1.0 / (1.0 + (-value).exp());
}
