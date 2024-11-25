use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};
use simple_example::circuit::MyCircuit;

fn main() {
    // ANCHOR: test-circuit
    // The number of rows in our circuit cannot exceed 2^k. Since our example
    // circuit is very small, we can pick a very small value here.
    let k = 4;  // TODO: why would it exceed 2^k?

    // Prepare the private and public inputs to the circuit!
    let constant = Fp::from(7);
    let a = Fp::from(2);
    let b = Fp::from(3);
    let c = (constant * a * b).square();

    // Instantiate the circuit with the private inputs
    let circuit = MyCircuit::new(constant, Value::known(a), Value::known(b));

    // Arrange the public input. We expose the multiplication result in row 0
    // of the instance column, so we position it there in our public inputs.
    let mut public_inputs = vec![c];

    // Given the correct public inputs, our circuit will verify
    let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    // If we try some other public input, the proof will fail!
    public_inputs[0] += Fp::one();
    let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
    assert!(prover.verify().is_err());
}
