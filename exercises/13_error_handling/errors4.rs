#![allow(clippy::comparison_chain)]

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // if value<0{
        //     Err(CreationError::Negative)        
        // }else if value==0 {
        //     Err(CreationError::Zero)     
        // }else {
        //     Ok(Self(value as u64))
        // }
        match value{
            1.. => Ok(Self(value as u64)),
            0=>Err(CreationError::Zero)  ,
            _=>Err(CreationError::Negative) 
        }
        // match value {
        //     x if x < 0 => Err(CreationError::Negative),
        //     0 => Err(CreationError::Zero),
        //     x => Ok(PositiveNonzeroInteger(x as u64)),
        // }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}