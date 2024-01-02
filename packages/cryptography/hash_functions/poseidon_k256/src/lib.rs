use ff::PrimeField;

pub struct PoseidonConstants<F: PrimeField>{
    pub round_keys: Vec<F>,
    pub mds_matrix: Vec<Vec<F>>,
    pub num_full_rounds: usize,
    pub num_partial_rounds: usize,
}

impl<F: PrimeField> PoseidonConstants<F> {
    pub fn new(
        round_keys: Vec<F>,
        mds_matrix: Vec<Vec<F>>,
        num_full_rounds: usize,
        num_partial_rounds: usize
    ) -> Self {
        Self {
            round_keys, 
            mds_matrix,
            num_full_rounds,
            num_partial_rounds
        }
    }
}


pub struct Poseidon<F: PrimeField> {
    pub state: [F; 3],
    pub constants: PoseidonConstants<F>,
    pub pos: usize
}

impl<F: PrimeField> Poseidon<F>{
    pub fn new(constants: PoseidonConstants<F>) -> Self {
        let state = [F::ZERO; 3];
        Self {
            state,
            constants,
            pos: 0
        }
    }

    pub fn hash(&mut self, input: &[F; 2]) -> F {
        let domain_tag = F::from(3);
        let input = [domain_tag, input[0], input[1]];

        self.state = input;

        let full_rounds_half = self.constants.num_full_rounds / 2;

        // First half of the full rounds
        for _ in 0..full_rounds_half {
            self.full_round();
        }

        // Partial rounds
        for _ in 0..self.constants.num_partial_rounds  {
            self.partial_rounds();
        }

        // Second half of full rounds
        for _ in 0..full_rounds_half {
            self.full_round();
        }

        self.state[1]
    }

    fn add_constants(&mut self) {
        // Add round constants 
        for i in 0..self.state.len() {
            self.state[i] += self.constants.round_keys[i + self.pos];
        }
    }

    fn matrix_mul(&mut self) {
        let mut result = [F::ZERO; 3];

        for (i, val) in self.constants.mds_matrix.iter().enumerate() {
            let mut tmp = F::ZERO;

            for (j, element) in self.state.iter().enumerate() {
                tmp += val[j] * element;
            }
            result[i] = tmp
        }

        self.state = result;
    }

    fn full_round(&mut self) {
        let t = self.state.len();
        self.add_constants();

        // S-boxes
        for i in 0..t {
            self.state[i] = self.state[i].pow_vartime(&[5, 0, 0, 0]);
        }

        self.matrix_mul();

        // Update the position of the round constants that are added
        self.pos += self.state.len();
    }

    fn partial_rounds(&mut self) {
        self.add_constants();

        // S-box
        self.state[0] = self.state[0].pow_vartime(&[5, 0, 0, 0]);

        self.matrix_mul();

        self.pos += self.state.len();
    }
}
