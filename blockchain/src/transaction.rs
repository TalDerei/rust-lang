use super::*;
use std::collections::HashSet;

// List of unspent notes
#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

// List of transactions
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Hashable for Output {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs
        .iter()
        .map(|output| output.value)
        .sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
        .iter()
        .map(|input| input.hash())
        .collect::<HashSet<Hash>>()
    }

    pub fn outputs_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}