use ironfish_bellperson::gadgets::boolean::Boolean;
use ironfish_bellperson::gadgets::sha256::sha256;
use ironfish_bellperson::{ConstraintSystem, SynthesisError};
use ff::PrimeField;

pub fn note_comm<Scalar, CS>(
    cs: CS,
    a_pk: &[Boolean],
    value: &[Boolean],
    rho: &[Boolean],
    r: &[Boolean],
) -> Result<Vec<Boolean>, SynthesisError>
where
    Scalar: PrimeField,
    CS: ConstraintSystem<Scalar>,
{
    assert_eq!(a_pk.len(), 256);
    assert_eq!(value.len(), 64);
    assert_eq!(rho.len(), 256);
    assert_eq!(r.len(), 256);

    let mut image = vec![
        Boolean::constant(true),
        Boolean::constant(false),
        Boolean::constant(true),
        Boolean::constant(true),
        Boolean::constant(false),
        Boolean::constant(false),
        Boolean::constant(false),
        Boolean::constant(false),
    ];
    image.extend(a_pk.iter().cloned());
    image.extend(value.iter().cloned());
    image.extend(rho.iter().cloned());
    image.extend(r.iter().cloned());

    sha256(cs, &image)
}
