mod layer;
mod neuron;

use rand::{RngCore, Rng};
use std::iter::once;
use self::layer::*; 



#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self::new(layers)
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let mut built_layers = Vec::new();

        for i in 0..(layers.len() - 1) {
            let input_size = layers[i].neurons;
            let output_size = layers[i + 1].neurons;

            built_layers.push(Layer::random(rng, input_size, output_size));
        }

        Self {
            layers: built_layers,
        }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propagate(inputs);
        }

        inputs
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::neuron::Neuron;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_neuron() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.8369198, 0.9090631, 0.6314245, 0.76194036].as_ref()
        );
    }

    #[test]
    fn propagete_neuron() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        //Ensures .max() workss
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }

    #[test]
    fn random_layer() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let layer = Layer::random(&mut rng, 4, 1);

        assert_eq!(layer.neurons.len(), 1);
        assert_eq!(layer.neurons[0].weights.len(), 4);

        assert_relative_eq!(layer.neurons[0].bias, -0.6255188);
        assert_relative_eq!(
            layer.neurons[0].weights.as_slice(),
            [0.8369198, 0.9090631, 0.6314245, 0.76194036].as_ref()
        );
    }

    #[test]
    fn propagete_layer() {
        let layer = Layer {
            neurons: vec![
                Neuron {
                    bias: 0.5,
                    weights: vec![-0.563, 0.823],
                },
                Neuron {
                    bias: 0.8,
                    weights: vec![-0.3673, 0.823],
                },
                Neuron {
                    bias: 0.4,
                    weights: vec![-0.3, 0.8],
                },
                Neuron {
                    bias: 0.123,
                    weights: vec![-0.45, 0.834],
                },
            ],
        };

        assert_eq!(
            layer.propagate([0.3, 1.0].to_vec()),
            [1.1541, 1.51281, 1.11, 0.822].to_vec()
        );
    }

    #[test]
    fn random_network() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let topo = vec![LayerTopology { neurons: 2 }, LayerTopology { neurons: 4 }];

        let network = Network::random(&mut rng, &topo);

        assert_eq!(network.layers[0].neurons.len(), 4);
        assert_eq!(network.layers[0].neurons[0].weights.len(), 2);

        assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
        assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            [0.8369198, 0.9090631].as_ref()
        );
    }

    #[test]
    fn propagate_network() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let topo = vec![LayerTopology { neurons: 2 }, LayerTopology { neurons: 4 }];

        let network = Network::random(&mut rng, &topo);
        let result = network.propagate(vec![1.0, 0.5]);

        assert_eq!(result, [0.66593254, 1.1409972, 0.41133577, 0.0]);
    }
}
