use perm::{Action, Table};
use rand::{distributions::Standard, prelude::Distribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Graph<const N: usize> {
    adjacencies: [[bool; N]; N],
}

impl<const N: usize> Distribution<Graph<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Graph<N> {
        let mut g = Graph::disconnected();
        for i in 0..N {
            for j in 0..N {
                g.adjacencies[i][j] = rng.sample(Standard);
            }
        }
        g
    }
}

impl<'a, const N: usize> Deserialize<'a> for Graph<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let mut graph = Graph::disconnected();
        let v: Vec<Vec<bool>> = Vec::deserialize(deserializer)?;
        for (iv, i) in v.into_iter().zip(0..) {
            if iv.len() != N {
                return Err(<D::Error as serde::de::Error>::custom::<&str>(
                    "not enough input",
                ));
            }
            for (b, j) in iv.into_iter().zip(0..) {
                graph.adjacencies[i][j] = b;
            }
        }

        Ok(graph)
    }
}

impl<const N: usize> Serialize for Graph<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v: Vec<Vec<bool>> = self
            .adjacencies
            .iter()
            .map(|b| b.iter().copied().collect())
            .collect();
        v.serialize(serializer)
    }
}

impl<const N: usize> Action<Graph<N>> for Table<N> {
    fn act(&self, element: &Graph<N>) -> Graph<N> {
        let mut g_prime = Graph::disconnected();
        for (connections, i) in element.adjacencies.iter().zip(0usize..) {
            for (edge, j) in connections.iter().zip(0usize..) {
                g_prime.set(self.act(&i), self.act(&j), *edge);
            }
        }
        g_prime
    }
}

impl<const N: usize> Graph<N> {
    pub fn disconnected() -> Graph<N> {
        Graph {
            adjacencies: [[false; N]; N],
        }
    }

    pub fn set(&mut self, i: usize, j: usize, e: bool) {
        self.adjacencies[i][j] = e;
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        self.adjacencies[i][j]
    }
}
