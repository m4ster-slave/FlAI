use crate::*;

#[derive(Debug)]
pub struct Layer {
    pub(crate) neurons: Vec<self::neuron::Neuron>,
}

impl Layer {
    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| self::neuron::Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let mut neurons = Vec::new();

        for _ in 0..output_size {
            neurons.push(self::neuron::Neuron::random(rng, input_size));
        }

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
}
