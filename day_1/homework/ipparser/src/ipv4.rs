pub struct Ipv4 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}

impl Ipv4 {
    pub fn show(self) {
        println!("IpAddr({}, {}, {}, {})", self.a, self.b, self.c, self.d);

        match self.range_type() {
            Some(kind) => println!("range: {}", kind),
            None => println!("range: ordinary address"),
        }
    }

    // Convert four bytes into one 32-bit number
    fn as_u32(&self) -> u32 {
        u32::from_be_bytes([self.a, self.b, self.c, self.d])
    }

    // Check if an address belongs to a network prefix
    fn matches_prefix(address: u32, network: u32, prefix: u32) -> bool {
        let mask = if prefix == 0 {
            0
        } else {
            u32::MAX << (32 - prefix)   //Shift 11111<-..00
        };

        // Compare the network parts of both addresses (compare every bit)
        (address & mask) == (network & mask)
    }

    pub fn range_type(&self) -> Option<&'static str> {
        let address = self.as_u32();

        let ranges: &[(u32, u32, &str)] = &[
            (0x00000000, 8, "current network / unspecified"),
            (0x0a000000, 8, "private"),
            (0x64400000, 10, "shared address space (CGNAT)"),
            (0x7f000000, 8, "loopback"),
            (0xa9fe0000, 16, "link-local"),
            (0xac100000, 12, "private"),
            (0xc0000000, 24, "protocol assignments"),
            (0xc0000200, 24, "documentation"),
            (0xc0586300, 24, "deprecated 6to4 relay"),
            (0xc0a80000, 16, "private"),
            (0xc6120000, 15, "benchmarking"),
            (0xc6336400, 24, "documentation"),
            (0xcb007100, 24, "documentation"),
            (0xe0000000, 4, "multicast"),
            (0xffffffff, 32, "limited broadcast"),
            (0xf0000000, 4, "reserved"),
        ];

        // Check the address against each network
        for &(network, prefix, kind) in ranges {
            if Self::matches_prefix(address, network, prefix) {
                return Some(kind);
            }
        }
        None
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
