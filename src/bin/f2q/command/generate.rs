use f2q::{
    code::{
        fermions::{
            An,
            Cr,
            Fermions,
            Orbital,
        },
        qubits::Paulis,
    },
    terms::SumRepr,
};
use rand::Rng;

use super::serialize_sumrepr;
use crate::{
    cli::Generate,
    errors::Error,
};

pub fn fermions(args: &Generate) -> Result<(), Error> {
    if args.random {
        fermions_random(args)
    } else {
        todo!()
    }
}

fn fermions_random(args: &Generate) -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let capacity = if let Ok(cap) = usize::try_from(args.num_terms) {
        cap
    } else {
        usize::MAX
    };
    let mut out_repr = SumRepr::with_capacity(capacity);
    let mut count = 0;
    while count < args.num_terms {
        let category = rng.gen_range(0..=2);
        match category {
            0 => out_repr.add_term(Fermions::Offset, rng.gen_range(-1.0..1.0)),
            1 => {
                let max_val = args.max_orbital_index;
                let p = rng.gen_range(0..max_val - 1);
                let q = rng.gen_range(p + 1..=max_val);
                out_repr.add_term(
                    Fermions::one_electron(
                        Cr(Orbital::with_index(p)),
                        An(Orbital::with_index(q)),
                    )
                    .unwrap(),
                    rng.gen_range(-1.0..1.0),
                );
            }
            2 => {
                let max_val = args.max_orbital_index;
                let p = rng.gen_range(0..max_val - 2);
                let q = rng.gen_range(p + 1..=max_val);
                let s = rng.gen_range(p..max_val - 1);
                let r = rng.gen_range(s + 1..=max_val);

                out_repr.add_term(
                    Fermions::two_electron(
                        (
                            Cr(Orbital::with_index(p)),
                            Cr(Orbital::with_index(q)),
                        ),
                        (
                            An(Orbital::with_index(r)),
                            An(Orbital::with_index(s)),
                        ),
                    )
                    .unwrap(),
                    rng.gen_range(-1.0..1.0),
                );
            }
            _ => unimplemented!(),
        }
        count += 1;
    }
    serialize_sumrepr(
        &out_repr,
        args.output_file.as_deref(),
        args.format,
        args.pretty_print,
    )
}

pub fn qubits(args: &Generate) -> Result<(), Error> {
    if args.random {
        qubits_random(args)
    } else {
        todo!()
    }
}

fn qubits_random(args: &Generate) -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let capacity = if let Ok(cap) = usize::try_from(args.num_terms) {
        cap
    } else {
        usize::MAX
    };
    let mut out_repr = SumRepr::with_capacity(capacity);
    for _ in 0..args.num_terms {
        out_repr.add_term(
            Paulis::new((rng.gen(), rng.gen())),
            rng.gen_range(-1.0..1.0),
        );
    }
    serialize_sumrepr(
        &out_repr,
        args.output_file.as_deref(),
        args.format,
        args.pretty_print,
    )
}
