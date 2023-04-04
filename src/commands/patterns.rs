use super::custom::rand_from_pattern;


enum CountryCode {
    US,
    EU,
    PL,
    CA,
    DE,
    ES,
    FI,
    FR,
    IT,
    NL,
    PT,
    RO,
    RU,
    SE,
    SG,
    SI,
    SK,
    ZA,
    ZM,
    UNKNOWN
}

enum Pattern {
    Unknown,
    PhoneNumber(CountryCode),
    Email,
    Url,
    Ssn,
    ZipCode,
    Currency,
    CountryCode,
    Latitude,
    Longitude,
    // CreditCard,
    // CreditCardNumber,
    // CreditCardExpiry,
    // CreditCardCvv,
    // CreditCardHolderName,
    // CreditCardSecurityCode,
    // CreditCardZipCode,
    // CreditCardExpirationDate,
    // CreditCardFirstName,
    // CreditCardLastName,
    // CreditCardAddress,
    // CreditCardCity,
    // CreditCardState,
    // CreditCardCountry,
    // CreditCardZipCodeState,
}

impl Default for Pattern {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<String> for Pattern {
    fn from(value: String) -> Self {
        match value {
            _ => Self::default(),
        }
    }
}
impl Into<String> for Pattern {
    fn into(self) -> String {
        match self {
            Pattern::Email => rand_from_pattern("${A:5-20}@${L:3-10}.com"),
            Pattern::PhoneNumber(cc) => rand_from_pattern(match cc {
                CountryCode::US => "+1-(${N:3})-${N:3}-${N:4}",
                CountryCode::PL => "+48 ${N:3}-${N:3}-${N:3}",
                CountryCode::UNKNOWN => "(${N:3})-",
                _ => "(${N:3})-",
            }),
            _ => rand_from_pattern(""),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_email() {
        let pat = Pattern::Email;
        let rand_email: String = pat.into();

        println!("Email: {}", rand_email);
    }

    #[test]
    fn test_pattern_phone_number() {
        for _ in 0..10 {

            let pat = Pattern::PhoneNumber(CountryCode::US);
            let rand_phone_number: String = pat.into();
    
            println!("Phone Number US: {}", rand_phone_number);
        }
    }
}