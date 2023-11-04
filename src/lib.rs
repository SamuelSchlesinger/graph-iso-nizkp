pub mod graph;

use graph::Graph;
use perm::{Action, Table};
use rand::{distributions::Standard, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub struct GraphIsomorphismPf<const N: usize> {
    a: Graph<N>,
    b: Graph<N>,
    cs: Vec<Graph<N>>,
    isos: Box<[Table<N>]>,
}

pub fn prove<const N: usize>(
    a: Graph<N>,
    b: Graph<N>,
    iso: Table<N>,
    m: usize,
) -> GraphIsomorphismPf<N> {
    if iso.act(&a) != b {
        panic!("liar");
    }

    let cs_and_their_isos: Vec<(Graph<N>, Table<N>)> = (0..m)
        .map(|_| {
            let table: Table<N> = rand::random();
            (table.act(&a), table)
        })
        .collect();

    let cs = cs_and_their_isos
        .iter()
        .map(|(a, _b)| a.clone())
        .collect::<Vec<_>>();

    let ctx = bincode::serialize(&(
        &a,
        &b,
        cs_and_their_isos
            .iter()
            .map(|(a, _b)| a)
            .collect::<Vec<_>>(),
    ))
    .expect("can serialize normal stuff");
    let seed = blake3::hash(&ctx).as_bytes().clone();
    eprintln!("seed: {:?}", seed);
    let mut rng = ChaCha20Rng::from_seed(seed);
    let bools = (0..m).map(|_| rng.sample(Standard)).collect::<Vec<bool>>();
    let isos = (0..m)
        .map(|i| {
            if bools[i] {
                cs_and_their_isos[i].1.invert()
            } else {
                iso.clone() * cs_and_their_isos[i].1.invert()
            }
        })
        .collect();

    GraphIsomorphismPf { a, b, cs, isos }
}

pub fn verify<const N: usize>(pf: &GraphIsomorphismPf<N>) -> bool {
    if pf.cs.len() != pf.isos.len() {
        return false;
    }
    let m = pf.cs.len();
    let ctx = bincode::serialize(&(&pf.a, &pf.b, &pf.cs)).expect("can serialize normal stuff");
    let seed = blake3::hash(&ctx).as_bytes().clone();
    eprintln!("seed: {:?}", seed);
    let mut rng = ChaCha20Rng::from_seed(seed);
    let bools = (0..m).map(|_| rng.sample(Standard)).collect::<Vec<bool>>();

    bools
        .into_iter()
        .zip(0..)
        .map(|(b, i)| {
            if b {
                pf.isos[i].act(&pf.cs[i]) == pf.a
            } else {
                pf.isos[i].act(&pf.cs[i]) == pf.b
            }
        })
        .all(|x| x)
}

#[test]
fn examples() {
    let mut a: Graph<3> = Graph::disconnected();
    a.set(0, 1, true);
    let mut b = Graph::disconnected();
    b.set(1, 0, true);
    let t = Table::swap(0, 1);
    let pf = prove(a, b, t, 20);
    assert!(verify(&pf))
}
