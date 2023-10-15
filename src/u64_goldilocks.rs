use lambdaworks_math::{
    field::traits::IsField,
    field::traits::IsPrimeField,
    field::errors::FieldError,
    errors::*,
    traits::*
};

pub const MODULUS: u64 = 0xffff_ffff_0000_0001;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GoldilocksField {
    value: u64,
}

impl GoldilocksField {

    fn from(a: u64) -> Self {
        if a >= MODULUS {
            GoldilocksField { value: a % MODULUS }
        } else {
            GoldilocksField { value: a }
        }
    }

    fn new(a: u64) -> u64 {
        if a >= MODULUS {
            a % MODULUS  // Take the value modulo MODULUS to keep it within the range
        } else {
            a
        }
    }

    fn generator() -> u64 {
        7
    }
}

impl IsField for GoldilocksField {
    type BaseType = u64;

    fn add(a: &u64, b: &u64) -> u64 {
        (*a + *b) % MODULUS
    }

    fn sub(a: &u64, b: &u64) -> u64 {
        (*a + MODULUS - *b) % MODULUS
    }

    fn neg(a: &u64) -> u64 {
        (MODULUS - *a) % MODULUS
    }

    fn mul(a: &u64, b: &u64) -> u64 {
        (*a * *b) % MODULUS
    }

    fn div(a: &u64, b: &u64) -> u64 {
        Self::mul(a, &Self::inv(b).unwrap())
    }

    fn inv(a: &u64) -> Result<u64, FieldError> {
        if *a == 0 {
            return Err(FieldError::InvZeroError);
        }
        Ok(Self::pow(a, MODULUS - 2))
    }

    fn eq(a: &u64, b: &u64) -> bool {
        Self::from_u64(*a) == Self::from_u64(*b)
    }

    fn zero() -> u64 {
        0
    }

    fn one() -> u64 {
        1
    }

    fn from_u64(x: u64) -> u64 {
        x % MODULUS
    }

    fn from_base_type(x: u64) -> u64 {
        Self::from_u64(x)
    }

}

impl IsPrimeField for GoldilocksField {
    type RepresentativeType = u64;

    fn representative(a: &Self::BaseType) -> Self::RepresentativeType {
        *a
    }

    fn field_bit_size() -> usize {
        ((MODULUS - 1).ilog2() + 1) as usize
    }

    fn from_hex(hex_string: &str) -> Result<Self::BaseType, CreationError> {
        let mut hex_string = hex_string;
        if hex_string.len() > 2 && &hex_string[..2] == "0x" {
            hex_string = &hex_string[2..];
        }

        u64::from_str_radix(hex_string, 16).map_err(|_| CreationError::InvalidHexString)
    }

}

impl ByteConversion for GoldilocksField {

    //#[cfg(feature = "std")]
    fn to_bytes_be(&self) -> Vec<u8> {
        u64::to_be_bytes(self.value).into()
    }

    //#[cfg(feature = "std")]
    fn to_bytes_le(&self) -> Vec<u8> {
        u64::to_le_bytes(self.value).into()
    }

    fn from_bytes_be(bytes: &[u8]) -> Result<Self, ByteConversionError> {
        let bytes: [u8; 8] = bytes[0..8].try_into().map_err(|_| ByteConversionError::FromBEBytesError)?; 
        let value = u64::from_be_bytes(bytes);
        Ok(Self { value }) 
    }

    fn from_bytes_le(bytes: &[u8]) -> Result<Self, ByteConversionError> {
        let bytes: [u8; 8] = bytes[0..8].try_into().map_err(|_| ByteConversionError::FromLEBytesError)?;
        let value = u64::from_le_bytes(bytes);
        Ok(Self { value }) 
    }
    

    
}

impl Serializable for GoldilocksField {
    fn serialize(&self) -> Vec<u8> {
        self.to_bytes_be()
    }
}

impl Deserializable for GoldilocksField {
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError>
        where
            Self: Sized {
                Self::from_bytes_be(bytes).map_err(|x| x.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;  
    const MODULUS: u64 = 0xffff_ffff_0000_0001;


    #[test]
    fn get_generator() {
        assert_eq!(GoldilocksField::generator(), 7);
    }

    #[test]
    fn test_add() {
        let a = 7;
        let b = 10;
        assert_eq!((GoldilocksField::add(&a, &b)), 17);
    }

    #[test]
    fn max_order_plus_1_is_0() {
        let a = MODULUS - 1;
        let b = 1;
        assert_eq!(GoldilocksField::add(&a, &b), 0);
    }

    #[test]
    fn test_sub() {
        let a = 5;
        let b = 3;
        assert_eq!(GoldilocksField::sub(&a, &b), 2);
    }

    #[test]
    fn test_neg() {
        let a = 5;
        let b = 3;
        assert_eq!(GoldilocksField::mul(&a, &b), 15);
    }

    #[test]
    fn test_mul() {
        let a = 5;
        let b = 3;
        assert_eq!(GoldilocksField::mul(&a, &b), 15);
    }

    #[test]
    fn mul_order_minus_1() {
        let a  = MODULUS - 1;
        let b  = MODULUS - 1;
        assert_eq!(GoldilocksField::mul(&a, &b), 1);
    }

    #[test]
    fn inv_0_error() {
        let a = 0;
        let result = GoldilocksField::inv(&a);
        assert!(matches!(result, Err(FieldError::InvZeroError)));
    }
    

    #[test]
    fn inv_2() {
        let a = 2;
        let modulus = MODULUS; // Assuming you have MODULUS defined somewhere
        let inverse_result = GoldilocksField::inv(&a).unwrap();
    
        // Check if a * inv(a) is congruent to 1 modulo MODULUS
        let product = GoldilocksField::mul(&modulus, &inverse_result);
        assert_eq!(product, 1);
    }


    #[test]
    fn pow_2_3() {
        assert_eq!(GoldilocksField::new(2).pow(3), GoldilocksField::new(8))
    }

    #[test]
    fn pow_p_minus_1() {
        assert_eq!(GoldilocksField::new(2).pow((MODULUS - 1) as u32), GoldilocksField::new(1))
    }

    #[test]
    fn div_1() {
        assert_eq!(GoldilocksField::new(2) / GoldilocksField::new(1), GoldilocksField::new(2))
    }

    #[test]
    fn div_4_2() {
        assert_eq!(GoldilocksField::new(4) / GoldilocksField::new(2), GoldilocksField::new(2))
    }

    #[test]
    fn div_4_3() {
        assert_eq!(GoldilocksField::new(4) / GoldilocksField::new(3) * GoldilocksField::new(3), GoldilocksField::new(4))
    }


    #[test]
    fn two_plus_its_additive_inv_is_0() {
        let two = GoldilocksField::new(2);
        assert_eq!((two - two), GoldilocksField::new(0))
    }

    #[test]
    fn four_minus_three_is_1() {
        let four = GoldilocksField::new(4);
        let three = GoldilocksField::new(3);

        assert_eq!(four - three, GoldilocksField::new(1))
    }

    #[test]
    fn zero_minus_1_is_order_minus_1() {
        let zero = GoldilocksField::new(0);
        let one = GoldilocksField::new(1);

        assert_eq!(zero - one, GoldilocksField::new(MODULUS - 1))
    }


    #[test]
    fn zero_constructor_returns_zero() {
        assert_eq!(GoldilocksField::new(0), GoldilocksField::new(0));
    }

    #[test]
    fn creating_a_field_element_from_its_representative_returns_the_same_element_1() {
        let change = 1;
        let f1 = GoldilocksField::from(MODULUS + change);
        let f2 = GoldilocksField::from(GoldilocksField::representative(&f1.value)); 
        assert_eq!(f1, f2);
    }
    
    #[test]
    fn creating_a_field_element_from_its_representative_returns_the_same_element_2() {
        let change = 8;
        let f1 = GoldilocksField::from(MODULUS + change);
        let f2 = GoldilocksField::from(GoldilocksField::representative(&f1.value)); 
        assert_eq!(f1, f2);
    }
    

}
