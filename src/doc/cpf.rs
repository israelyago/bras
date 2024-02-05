use core::fmt::Display;
use core::str::FromStr;

/// # Examples
/// ```
/// use bras::Cpf;
/// use core::str::FromStr;
/// # use bras::ParseCpfError;
///
/// let cpf = Cpf::from_str("98484485439")?;
/// assert_eq!("984.844.854-39", cpf.to_string());
///
/// let cpf = Cpf::from_str("984.844.854-39")?;
/// assert_eq!("984.844.854-39", cpf.to_string());
///
/// # Ok::<(), ParseCpfError>(())
/// ```
///
/// ## Conversions
/// ```
/// use bras::Cpf;
/// # use bras::ParseCpfError;
///
/// let cpf = "984.844.854-39".parse::<Cpf>()?;
/// assert_eq!("984.844.854-39", cpf.to_string());
///
/// let cpf: Cpf = "984.844.854-39".parse()?;
/// assert_eq!("98484485439", cpf.numbers_as_string());
///
/// let cpf: Cpf = "984.844.854-39".parse()?;
/// assert_eq!(98484485439u64, u64::from(cpf));
///
/// let cpf: Cpf = "98484485439".parse()?;
/// assert_eq!(String::from("984.844.854-39"), String::from(cpf));
///
/// let number: u64 = 98484485439;
/// let cpf: Cpf = Cpf::try_from(98484485439)?;
///
/// assert_eq!("984.844.854-39", cpf.to_string());
///
/// # Ok::<(), ParseCpfError>(())
/// ```
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Cpf {
    inner: u64,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[non_exhaustive]
pub enum ParseCpfError {
    Invalid,
}

impl FromStr for Cpf {
    type Err = ParseCpfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cpf::new(s.into())
    }
}

impl From<Cpf> for String {
    fn from(cpf: Cpf) -> Self {
        let value_as_str = format!("{:011}", cpf.inner);
        let a = &value_as_str[0..3];
        let b = &value_as_str[3..6];
        let c = &value_as_str[6..9];
        let d = &value_as_str[9..];
        format!("{}.{}.{}-{}", a, b, c, d)
    }
}

impl From<Cpf> for u64 {
    /// ```
    /// use bras::Cpf;
    /// # use bras::ParseCpfError;
    ///
    /// let cpf: Cpf = "984.844.854-39".parse()?;
    /// assert_eq!(98484485439u64, u64::from(cpf));
    /// # Ok::<(), ParseCpfError>(())
    /// ```
    fn from(cpf: Cpf) -> Self {
        cpf.inner
    }
}

impl TryFrom<u64> for Cpf {
    type Error = ParseCpfError;

    /// ```
    /// use bras::Cpf;
    /// # use bras::ParseCpfError;
    ///
    /// let cpf: Cpf = Cpf::try_from(1678346063)?;
    /// assert_eq!("016.783.460-63", cpf.to_string());
    /// 
    /// # Ok::<(), ParseCpfError>(())
    /// ```
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Cpf::new(format!("{:011}", value))
    }
}

impl Display for Cpf {
    /// ```
    ///  use bras::Cpf;
    /// # use bras::ParseCpfError;
    ///
    /// let cpf: Cpf = "98484485439".parse()?;
    /// assert_eq!(String::from("984.844.854-39"), String::from(cpf));
    ///
    /// # Ok::<(), ParseCpfError>(())
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formated = String::from(*self);
        f.write_str(&formated)
    }
}

const FIRST_DIGIT_ARRAY: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
const SECOND_DIGIT_ARRAY: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

impl Cpf {
    pub fn numbers_as_string(self) -> String {
        self.inner.to_string()
    }

    fn new(s: String) -> Result<Self, ParseCpfError> {
        if s.len() != 11 && s.len() != 14 {
            return Err(ParseCpfError::Invalid);
        }
        if s.len() == 14 {
            let c: Vec<char> = s.chars().collect();
            if c[3] != '.' || c[7] != '.' || c[11] != '-' {
                return Err(ParseCpfError::Invalid);
            }
        }
        let numbers: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
        if numbers.len() != 11 {
            return Err(ParseCpfError::Invalid);
        }
        let first_verifier_digit = &numbers[9];

        let all_equals = numbers.iter().all(|n| n == first_verifier_digit);
        if all_equals {
            return Err(ParseCpfError::Invalid);
        }

        Self::check_first_verifier_digit(&numbers, first_verifier_digit)?;

        let second_verifier_digit = &numbers[10];
        Self::check_second_verifier_digit(&numbers, second_verifier_digit)?;

        let value_as_string: String = numbers.iter().map(|n| n.to_string()).collect();
        let value: u64 = u64::from_str(&value_as_string).map_err(|_| ParseCpfError::Invalid)?;

        Ok(Cpf { inner: value })
    }

    fn check_first_verifier_digit(numbers: &[u32], got: &u32) -> Result<(), ParseCpfError> {
        let calculated = Cpf::first_verifier_digit(&numbers.to_vec());
        if got != &calculated {
            Err(ParseCpfError::Invalid)
        } else {
            Ok(())
        }
    }

    fn check_second_verifier_digit(numbers: &[u32], got: &u32) -> Result<(), ParseCpfError> {
        let calculated = Cpf::second_verifier_digit(&numbers.to_vec());
        if got != &calculated {
            Err(ParseCpfError::Invalid)
        } else {
            Ok(())
        }
    }

    fn first_verifier_digit(numbers: &Vec<u32>) -> u32 {
        let sum = FIRST_DIGIT_ARRAY
            .iter()
            .zip(numbers)
            .map(|pair| pair.0 * pair.1)
            .sum();
        Self::sum_to_digit(sum)
    }

    fn second_verifier_digit(numbers: &Vec<u32>) -> u32 {
        let sum = SECOND_DIGIT_ARRAY
            .iter()
            .zip(numbers)
            .map(|pair| pair.0 * pair.1)
            .sum();
        Self::sum_to_digit(sum)
    }

    fn sum_to_digit(sum: u32) -> u32 {
        let digit = sum * 10 % 11;
        if digit == 10 {
            0
        } else {
            digit
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cpf_from_str() {
        let cpf = Cpf::from_str("98484485439").unwrap();
        assert_eq!("984.844.854-39", cpf.to_string());

        let cpf = Cpf::from_str("984.844.854-39").unwrap();
        assert_eq!("984.844.854-39", cpf.to_string());

        let cpf = Cpf::from_str("05119439039").unwrap();
        assert_eq!("051.194.390-39", cpf.to_string());
    }

    #[test]
    fn return_error_on_invalid_str() {
        assert_eq!(
            Cpf::from_str("invalid_str").unwrap_err(),
            ParseCpfError::Invalid
        );
        assert_eq!(
            Cpf::from_str("98484485401").unwrap_err(),
            ParseCpfError::Invalid
        );
        assert_eq!(
            Cpf::from_str("98484485439invalid_str").unwrap_err(),
            ParseCpfError::Invalid
        );
        assert_eq!(
            Cpf::from_str("984-844-854.39").unwrap_err(),
            ParseCpfError::Invalid
        );
    }

    #[test]
    fn all_digits_the_same_is_an_invalid_cpf() {
        let invalid_cpfs_by_definition = [
            "00000000000",
            "11111111111",
            "22222222222",
            "33333333333",
            "44444444444",
            "55555555555",
            "66666666666",
            "77777777777",
            "88888888888",
            "99999999999",
        ];

        for cpf in invalid_cpfs_by_definition {
            assert_eq!(Cpf::from_str(cpf).unwrap_err(), ParseCpfError::Invalid);
        }
    }
}
