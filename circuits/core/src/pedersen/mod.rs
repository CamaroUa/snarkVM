// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod hash;

use snarkvm_algorithms::crypto_hash::hash_to_curve;
use snarkvm_circuits_environment::Mode;
use snarkvm_circuits_types::{Double, Environment, Group, Inject};

pub struct Pedersen<E: Environment, const NUM_WINDOWS: usize, const WINDOW_SIZE: usize> {
    pub bases: Vec<Vec<Group<E>>>,
}

impl<E: Environment, const NUM_WINDOWS: usize, const WINDOW_SIZE: usize> Pedersen<E, NUM_WINDOWS, WINDOW_SIZE> {
    #[allow(dead_code)]
    fn setup(message: &str) -> Self {
        let bases = (0..NUM_WINDOWS)
            .map(|index| {
                // Construct an indexed message to attempt to sample a base.
                let (generator, _, _) = hash_to_curve(&format!("{message} at {index}"));
                // Inject the new base.
                let mut base = Group::new(Mode::Constant, generator);
                let mut powers = Vec::with_capacity(WINDOW_SIZE);
                for _ in 0..WINDOW_SIZE {
                    powers.push(base.clone());
                    base = base.double();
                }
                powers
            })
            .collect();

        Self { bases }
    }

    #[allow(dead_code)]
    fn parameters(&self) -> Vec<Vec<Group<E>>> {
        self.bases.clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    fn check_setup() {
        // Check for equivalency of bases.
        native_hasher.parameters().iter().zip(circuit_hasher.parameters().iter()).for_each(
            |(native_bases, circuit_bases)| {
                native_bases.iter().zip(circuit_bases.iter()).for_each(|(native_base, circuit_base)| {
                    assert_eq!(native_base.into_affine(), circuit_base.eject_value())
                })
            },
        );
    }

    #[test]
    fn test_setup_constant {


    }
}
*/
