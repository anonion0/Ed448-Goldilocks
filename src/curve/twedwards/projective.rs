use crate::curve::edwards::ExtendedPoint;
use crate::field::FieldElement;
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable};

impl Default for ProjectiveNielsPoint {
    fn default() -> ProjectiveNielsPoint {
        ProjectiveNielsPoint::identity()
    }
}

// Its a variant of Niels, where a Z coordinate is added for unmixed readdition
// ((y+x)/2, (y-x)/2, dxy, Z)
#[derive(Copy, Clone)]
pub struct ProjectiveNielsPoint {
    pub(crate) Y_plus_X: FieldElement,
    pub(crate) Y_minus_X: FieldElement,
    pub(crate) Td: FieldElement,
    pub(crate) Z: FieldElement,
}

impl ConditionallySelectable for ProjectiveNielsPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        ProjectiveNielsPoint {
            Y_plus_X: FieldElement::conditional_select(&a.Y_plus_X, &b.Y_plus_X, choice),
            Y_minus_X: FieldElement::conditional_select(&a.Y_minus_X, &b.Y_minus_X, choice),
            Td: FieldElement::conditional_select(&a.Td, &b.Td, choice),
            Z: FieldElement::conditional_select(&a.Z, &b.Z, choice),
        }
    }
}
impl ConditionallyNegatable for ProjectiveNielsPoint {
    fn conditional_negate(&mut self, choice: Choice) {
        FieldElement::conditional_swap(&mut self.Y_minus_X, &mut self.Y_plus_X, choice);
        self.Td.conditional_negate(choice);
    }
}

impl ProjectiveNielsPoint {
    pub fn identity() -> ProjectiveNielsPoint {
        ProjectiveNielsPoint {
            Y_plus_X: FieldElement::one(),
            Y_minus_X: FieldElement::one(),
            Td: FieldElement::zero(),
            Z: FieldElement::one(),
        }
    }
    pub fn to_extended(&self) -> ExtendedPoint {
        let A = self.Y_plus_X - self.Y_minus_X;
        let B = self.Y_plus_X + self.Y_minus_X;
        ExtendedPoint {
            X: self.Z * A,
            Y: self.Z * B,
            Z: self.Z.square(),
            T: B * A,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use hex::decode as hex_decode;

    fn slice_to_fixed_array(b: &[u8]) -> [u8; 56] {
        let mut a: [u8; 56] = [0; 56];
        a.copy_from_slice(&b);
        a
    }

    fn hex_to_field(data: &str) -> FieldElement {
        let mut bytes = hex_decode(data).unwrap();
        bytes.reverse();
        FieldElement::from_bytes(&slice_to_fixed_array(&bytes))
    }

    #[test]
    fn test_conditional_negate() {
        let Y_minus_X = hex_to_field("4b8a632c1feab72769cd96e7aaa577861871b3613945c802b89377e8b85331ecc0ffb1cb20169bfc9c27274d38b0d01e87a1d5d851770bc8");
        let Y_plus_X = hex_to_field("81a45f02f41053f8d7d2a1f176a340529b33b7ee4d3fa84de384b750b35a54c315bf36c41d023ade226449916e668396589ea2145da09b95");
        let Td = hex_to_field("5f5a2b06a2dbf7136f8dc979fd54d631ca7de50397250a196d3be2a721ab7cbaa92c545d9b15b5319e11b64bc031666049d8637e13838b3b");
        let Z = FieldElement::one();

        let mut n = ProjectiveNielsPoint {
            Y_plus_X,
            Y_minus_X,
            Td,
            Z,
        };

        let expected_neg_n = ProjectiveNielsPoint {
            Y_plus_X: Y_minus_X,
            Y_minus_X: Y_plus_X,
            Td: Td.negate(),
            Z: Z,
        };

        n.conditional_negate(1.into());

        assert!(expected_neg_n.Y_plus_X == n.Y_plus_X);
        assert!(expected_neg_n.Y_minus_X == n.Y_minus_X);
        assert!(expected_neg_n.Td == n.Td);
    }
}
