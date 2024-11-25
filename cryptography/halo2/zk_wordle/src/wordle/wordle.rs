use group::ff::Field;
use halo2_proofs::plonk::{Advice, Column, Instance, Selector};

use self::{
    is_zero::IsZeroChip,
    table::DicTableConfig,
    utils::{WORD_COUNT, WORD_LEN},
};

mod is_zero;
mod table;
mod utils;

pub struct WordCheckConfig<F: Field> {
    // Selectors
    q_input: Selector,
    q_diff_g: Selector,
    q_diff_y: Selector,
    q_diff_green_is_zero: Selector,
    q_diff_yellow_is_zero: Selector,
    q_color_is_zero: Selector,
    q_color: Selector,

    // Advice columns
    poly_word: Column<Advice>,
    chars: [Column<Advice>; WORD_LEN],
    color_is_zero_advice_column: [Column<Advice>; WORD_LEN],

    // Selector columns
    final_word_chars_instance: Column<Instance>,
    char_green_instance: Column<Instance>,
    char_yellow_instance: Column<Instance>,

    // Chips
    table: DicTableConfig<F>,
    diffs_green_is_zero: [IsZeroChip<F>; WORD_LEN],
    diffs_yellow_is_zero: [IsZeroChip<F>; WORD_LEN],
}

#[cfg(test)]
mod tests {
    use std::str::Chars;

    use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp, plonk::Assigned};

    use super::*;

    #[test]
    fn test_wordle_1() {
        let k = 14;
        let words = [
            String::from("audio"),
            String::from("hunky"),
            String::from("fluff"),
            String::from("fluff"),
        ];

        let mut poly_words: [Value<Assigned<Fp>>; WORD_COUNT] =
            [Value::known(Fp::from(123).into()); WORD_COUNT];

        let mut words_chars: [[Value<Assigned<Fp>>; WORD_LEN]; WORD_COUNT] =
            [[Value::known(Fp::from(123).into()); WORD_LEN]; WORD_COUNT];

        for idx in 0..WORD_COUNT {
            poly_words[idx] = Value::known(Fp::from(word_to_polyhash(&words[idx].clone())).into());
            let chars = words_to_chars(&words[idx].clone());

            for i in 0..WORD_LEN {
                words_chars[idx][i] = Value::known(Fp::from(char[i]).into());
            }
        }

        let final_word = String::from("fluff");
        let final_chars = word_to_chars(&final_word);

        let mut word_diffs_green = [[Value::from(Fp::from(123).into()); WORD_LEN]; WORD_COUNT];
        let mut word_diffs_yellow = [[Value::known(Fp::from(123).into()); WORD_LEN]; WORD_COUNT];

        for idx in 0..WORD_COUNT {
            let chars = words_to_chars(&words[idx].clone());

            for i in 0..WORD_LEN {
                word_diffs_green[idx][i] =
                    Value::from((Fp::from(chars[i]) - Fp::from(final_word[i]).into()));
            }

            for i in 0..WORD_LEN {
                let yellow_diff = {
                    (0..WORD_LEN).fold(Fp::from(1), |expr, j| {
                        expr * (Fp::from(chars[i]) - Fp::from(final_chars[j]))
                    })
                };

                word_diffs_yellow[idx][i] = Value::from(Fp::from(yellow_diff).into());
            }
        }

        // Successful cases
        let circuit = WordleCircuit::<Fp> {
            poly_words,
            words_chars,
            words_diffs_green,
            words_diffs_yellow,
        };

        let mut instance = Vec::new();

        // Final words chars
        let mut final_chars_instance = vec![];
        for i in 0..WORD_LEN {
            final_chars_instance.push(Fp::from(final_chars[i]));
        }
        instance.push(final_chars_instance);

        let mut diffs = vec![];
        for idx in 0..WORD_COUNT {
            diffs.push(compute_diff(&words[idx], &final_word));
        }

        // Color green;
        let mut green = vec![];
        for idx in 0..WORD_COUNT {
            for i in 0..WORD_LEN {
                green.push(diffs[idx][0][i]);
            }
        }
        instance.push(green);

        // Color yellow
        let mut yellow = vec![];
        for idx in 0..WORD_COUNT {
            for i in 0..WORD_LEN {
                yellow.push(diffs[idx][1][i]);
            }
        }
        instance.push(yellow);

        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }
}
