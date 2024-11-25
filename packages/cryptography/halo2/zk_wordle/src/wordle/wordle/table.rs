use std::marker::PhantomData;

use group::ff::Field;
use halo2_proofs::{
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error, TableColumn},
};
use serde::{Deserialize, Serialize};

use crate::utils::dict::get_dict;

#[derive(Deserialize, Serialize)]
struct Dict {
    words: Vec<String>,
}

/// A lookup table of values from dictionary.
#[derive(Debug, Clone)]
pub struct DicTableConfig<F: Field> {
    pub(super) value: TableColumn,
    _marker: PhantomData<F>,
}

impl<F: Field> DicTableConfig<F>
where
    F: Field + From<u64>,
{
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();

        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// The &mut impl Layouter<F> syntax is used for a mutable reference to a trait object,
    /// allowing the method to work with any type that implements the Layouter trait,
    /// while also being able to modify the state of the Layouter instance.
    /// This is useful for polymorphism, where you want your function to accept any type that implements a specific trait,
    /// and you also need to mutate (change) the state of the object passed to the function.
    pub(super) fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        let mut words = get_dict();

        words.push(0); // TODO: why push 0?

        // In Halo2, a zero-knowledge proof library, `layouter.assign_table` and `layouter.assign_region` serve different purposes in circuit layout:
        //
        // `layouter.assign_region`:
        // - Assigns values to a specific region within a circuit.
        // - A region is a logical grouping of gates or constraints performing a specific function.
        // - It enables modular design, allowing for encapsulation of functionality and component reuse.
        //
        // `layouter.assign_table`:
        // - Assigns values to a lookup table within the circuit.
        // - Lookup tables pre-compute values for operations, reducing complexity by avoiding on-the-fly computations.
        // - This simplifies constraints for certain operations, enabling efficient implementation of complex functions within a zero-knowledge circuit.
        //
        // Differences:
        // - `assign_region` focuses on organizing and implementing circuit logic in modules.
        // - `assign_table` optimizes circuit performance and complexity with pre-computed values.
        // Both methods are crucial for designing efficient, maintainable, and optimized zero-knowledge proof circuits.
        layouter.assign_table(
            || "load dictionary-check table",
            |mut table| {
                let mut offset = 0;
                for word in words.iter() {
                    table.assign_cell(
                        || "num_bits",
                        self.value,
                        offset,
                        || Value::known(F::from(word.clone() as u64)),
                    )?;
                    offset += 1;
                }

                Ok(())
            },
        )
    }
}
