pub struct Ipv4 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}

impl Ipv4 {
    pub fn show(self) {
        println!("IpAddr({}, {}, {}, {})", self.a, self.b, self.c, self.d);
    }
}

pub enum ParseError {
    WrongFieldCount(usize, usize),
    InvalidNumber(usize),
    OutOfRange(usize, u16),
    LeadingZero(usize),
}

impl ParseError {
    pub fn describe(&self) {
        match self {
            ParseError::WrongFieldCount(expected, found) => {
                println!("error: expected {} fields, found {}", expected, found);
            }
            ParseError::InvalidNumber(field) => {
                println!("error: field {} is not a valid number", field);
            }
            ParseError::OutOfRange(field, value) => {
                println!("error: field {}: {} out of range 0-255", field, value);
            }
            ParseError::LeadingZero(field) => {
                println!("error: field {} has a leading zero", field);
            }
        }
    }
}

pub enum Outcome {
    Ok(Ipv4),
    Err(ParseError),
}

pub fn parse_ipv4(input: &str) -> Outcome {
    let fields: Vec<&str> = input.split('.').collect();

    if fields.len() != 4 {
        return Outcome::Err(ParseError::WrongFieldCount(4, fields.len()));
    }

    let mut numbers: Vec<u8> = Vec::new();

    for (i, field) in fields.iter().enumerate() {
        if field.len() > 1 && field.starts_with('0') {
            return Outcome::Err(ParseError::LeadingZero(i));
        }

        let number = match field.parse::<u16>() {
            Ok(n) => n,
            Err(_) => return Outcome::Err(ParseError::InvalidNumber(i)),
        };

        if number > 255 {
            return Outcome::Err(ParseError::OutOfRange(i, number));
        }

        numbers.push(number as u8);
    }

    Outcome::Ok(Ipv4 {
        a: numbers[0],
        b: numbers[1],
        c: numbers[2],
        d: numbers[3],
    })
}
