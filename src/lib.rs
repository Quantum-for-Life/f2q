use std::{
    collections::HashMap,
    fmt::Display,
};

use num::Float;

const PAULI_MASK: u128 = 3;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoCode,
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NoCode => write!(f, "NoCode"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub enum Pauli {
    #[default]
    I,
    X,
    Y,
    Z,
}

impl Pauli {
    /// # Panics
    ///
    /// Panics if value is outside 0..4
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        Self::try_from(value).expect("should be an integer between 0 and 3")
    }
}

macro_rules! impl_pauli_int {
    ( $($Typ:ty)* ) => {
        $(

            impl TryFrom<$Typ> for Pauli {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    use Pauli::*;
                    match value {
                        0 => Ok(I),
                        1 => Ok(X),
                        2 => Ok(Y),
                        3 => Ok(Z),
                        _ => Err(Self::Error::NoCode),
                    }
                }
            }

            impl From<Pauli> for $Typ {
                fn from(value: Pauli) -> Self {
                    match value {
                        Pauli::I => 0,
                        Pauli::X => 1,
                        Pauli::Y => 2,
                        Pauli::Z => 3,
                    }
                }
            }

        )*
    };
}

impl_pauli_int!(u8 u16 u32 u64 u128);
impl_pauli_int!(i8 i16 i32 i64 i128);

/// Pauli string of up to 64 qubits.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PauliCode {
    pack: u128,
}

impl Default for PauliCode {
    fn default() -> Self {
        Self::new(0)
    }
}

impl PauliCode {
    #[must_use]
    pub fn new(pack: u128) -> Self {
        Self {
            pack,
        }
    }

    #[must_use]
    pub fn as_u128(&self) -> &u128 {
        &self.pack
    }

    /// # Safety
    ///
    /// Make sure index is within 0..64
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub unsafe fn pauli_unchecked(
        &self,
        index: u8,
    ) -> Pauli {
        let pauli_int = (self.pack >> (index * 2)) & PAULI_MASK;
        Pauli::try_from(pauli_int).expect("incorrect encoding. This is a bug")
    }

    #[must_use]
    pub fn pauli(
        &self,
        index: u8,
    ) -> Option<Pauli> {
        if index >= 64 {
            None
        } else {
            // SAFETY: We just checked if index is within bounds
            Some(unsafe { self.pauli_unchecked(index) })
        }
    }

    /// # Safety
    ///
    /// Make sure index is within 0..64
    pub unsafe fn pauli_mut_unchecked<OP>(
        &mut self,
        index: u8,
        f: OP,
    ) where
        OP: FnOnce(&mut Pauli),
    {
        let mut pauli = self.pauli_unchecked(index);
        f(&mut pauli);
        self.pack &= !(PAULI_MASK << (index * 2));
        self.pack |= u128::from(pauli) << (index * 2);
    }

    pub fn pauli_mut<OP>(
        &mut self,
        index: u8,
        f: OP,
    ) where
        OP: FnOnce(Option<&mut Pauli>),
    {
        if index >= 64 {
            f(None);
        } else {
            // SAFETY: We just checked if index is within bounds
            unsafe {
                self.pauli_mut_unchecked(index, |x: &mut Pauli| f(Some(x)));
            }
        }
    }

    /// # Panics
    ///
    /// Panic is index outside of 0..64
    pub fn set(
        &mut self,
        index: u8,
        pauli: Pauli,
    ) {
        self.pauli_mut(index, |x| {
            if let Some(p) = x {
                *p = pauli;
            } else {
                panic!("index should be within 0..64");
            }
        });
    }

    #[must_use]
    pub fn iter(&self) -> Codes<'_> {
        Codes::new(self)
    }

    pub fn from_paulis<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Pauli>,
    {
        let pack = (0..32)
            .zip(iter)
            .fold(0, |acc, (i, pauli)| acc + (u128::from(pauli) << (i * 2)));
        Self::new(pack)
    }
}

impl<'a> IntoIterator for &'a PauliCode {
    type IntoIter = Codes<'a>;
    type Item = Pauli;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Iterate over Paulis in PauliCode
#[derive(Debug)]
pub struct Codes<'a> {
    code:  &'a PauliCode,
    index: u8,
}

impl<'a> Codes<'a> {
    fn new(code: &'a PauliCode) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl<'a> Iterator for Codes<'a> {
    type Item = Pauli;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }

        let pauli = self.code.pauli(self.index);
        self.index += 1;
        pauli
    }
}

#[derive(Debug)]
pub struct PauliHamil<T> {
    map: HashMap<PauliCode, T>,
}

impl<T> Default for PauliHamil<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PauliHamil<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn as_map(&self) -> &HashMap<PauliCode, T> {
        &self.map
    }

    pub fn as_map_mut(&mut self) -> &mut HashMap<PauliCode, T> {
        &mut self.map
    }
}

impl<T> PauliHamil<T>
where
    T: Float,
{
    #[must_use]
    pub fn coeff(
        &self,
        code: &PauliCode,
    ) -> T {
        match self.map.get(code) {
            Some(coeff) => *coeff,
            None => T::zero(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pauli_01() {
        assert_eq!(Pauli::try_from(0u128).unwrap(), Pauli::I);
        assert_eq!(Pauli::try_from(1u128).unwrap(), Pauli::X);
        assert_eq!(Pauli::try_from(2u128).unwrap(), Pauli::Y);
        assert_eq!(Pauli::try_from(3u128).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_pauli_02() {
        let err = Pauli::try_from(4u128).unwrap_err();
        assert_eq!(err, Error::NoCode);
    }

    #[test]
    fn test_pauli_03() {
        assert_eq!(u8::from(Pauli::I), 0);
        assert_eq!(u8::from(Pauli::X), 1);
        assert_eq!(u8::from(Pauli::Y), 2);
        assert_eq!(u8::from(Pauli::Z), 3);
    }

    #[test]
    fn test_paulicode_init() {
        let code = PauliCode::new(0b01);
        assert_eq!(*code.as_u128(), 0b01);
    }

    #[test]
    fn test_paulicode_pauli_02() {
        let code = PauliCode::new(0b0101);

        assert_eq!(code.pauli(0), Some(Pauli::X));
        assert_eq!(code.pauli(1), Some(Pauli::X));
        assert_eq!(code.pauli(2), Some(Pauli::I));
        assert_eq!(code.pauli(63), Some(Pauli::I));

        assert_eq!(code.pauli(64), None);
        assert_eq!(code.pauli(123), None);
    }

    #[test]
    fn test_paulicode_pauli_mut_01() {
        let mut code = PauliCode::default();
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.pauli_mut(7, |x| {
            if let Some(pauli) = x {
                *pauli = Pauli::Z;
            }
        });
        assert_eq!(code.pauli(7).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_paulicode_set_pauli_01() {
        let mut code = PauliCode::new(29_332_281_938);
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.set(7, Pauli::Y);
        assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
    }

    #[test]
    #[should_panic(expected = "index should be within 0..64")]
    fn test_paulicode_set_pauli_02() {
        let mut code = PauliCode::default();
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.set(65, Pauli::Y);
        assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
    }

    #[test]
    fn test_paulicode_set_pauli_03() {
        let mut code = PauliCode::default();

        for i in 0..13 {
            code.set(i, Pauli::X);
        }
        for i in 13..29 {
            code.set(i, Pauli::Y);
        }
        for i in 29..61 {
            code.set(i, Pauli::Z);
        }

        for i in 0..13 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::X, "{i}");
        }
        for i in 13..29 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::Y, "{i}");
        }
        for i in 29..61 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::Z, "{i}");
        }
        for i in 61..64 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::I, "{i}");
        }
    }

    #[test]
    fn test_paulicode_codes_iter_01() {
        use Pauli::*;
        let result = PauliCode::new(0b01).iter().take(3).collect::<Vec<_>>();

        assert_eq!(result, &[X, I, I]);
    }

    #[test]
    fn test_paulicode_codes_iter_02() {
        use Pauli::*;
        let result =
            PauliCode::new(0b11_1001).iter().take(5).collect::<Vec<_>>();

        assert_eq!(result, &[X, Y, Z, I, I]);
    }

    #[test]
    fn test_paulicode_from_paulis_01() {
        use Pauli::*;

        assert_eq!(
            PauliCode::from_paulis([I, X, Y, Z]),
            PauliCode::new(0b1110_0100)
        );
    }

    #[test]
    fn test_paulihamil_init_01() {
        let code = PauliCode::new(1234);
        let mut hamil = PauliHamil::new();

        hamil.as_map_mut().insert(code, 4321.);
        let coeff = hamil.coeff(&code);
        assert!(f64::abs(coeff - 4321.) < f64::EPSILON);
    }
}
