pub struct Ipv6 {
    pub groups: [u16; 8],
}

impl Ipv6 {
    pub fn show(&self) {
        print!("Ipv6(");

        for (i, group) in self.groups.iter().enumerate() {
            if i > 0 {
                print!(":");
            }
            print!("{:x}", group);
        }

        println!(")");

        match self.range_type() {
            Some(kind) => println!("range: {}", kind),
            None => println!("range: not in the checked special ranges"),
        }
    }

    fn as_u128(&self) -> u128 {
        self.groups
            .iter()
            .fold(0u128, |address, group| (address << 16) | u128::from(*group))
    }

    fn has_prefix(&self, network: u128, prefix: u32) -> bool {
        let address = self.as_u128();
        let shift = 128 - prefix;

        (address >> shift) == (network >> shift)
    }

    pub fn range_type(&self) -> Option<&'static str> {
        let ranges: &[(u128, u32, &str)] = &[
            (0x00000000000000000000000000000001, 128, "loopback"),
            (0x00000000000000000000000000000000, 128, "unspecified"),
            (0x00000000000000000000ffff00000000, 96, "IPv4-mapped"),
            (0x0064ff9b000000000000000000000000, 96, "NAT64 translation"),
            (0x01000000000000000000000000000000, 64, "discard-only"),
            (0x20010db8000000000000000000000000, 32, "documentation"),
            (0x20010000000000000000000000000000, 23, "special-purpose"),
            (
                0xfc000000000000000000000000000000,
                7,
                "unique-local / private",
            ),
            (0xfe800000000000000000000000000000, 10, "link-local"),
            (0xff000000000000000000000000000000, 8, "multicast"),
            (
                0x20000000000000000000000000000000,
                3,
                "global unicast range",
            ),
        ];

        for &(network, prefix, kind) in ranges {
            if self.has_prefix(network, prefix) {
                return Some(kind);
            }
        }

        None
    }
}

pub enum ParseError {
    InvalidFormat,
    InvalidGroup(usize),
    WrongGroupCount(usize),
}

impl ParseError {
    pub fn describe(&self) {
        match self {
            ParseError::InvalidFormat => println!("error: invalid IPv6 format"),
            ParseError::InvalidGroup(group) => {
                println!("error: invalid IPv6 group {}", group);
            }
            ParseError::WrongGroupCount(count) => {
                println!("error: expected 8 groups, found {}", count);
            }
        }
    }
}

pub enum Outcome {
    Ok(Ipv6),
    Err(ParseError),
}

fn parse_groups(part: &str, offset: usize) -> Result<Vec<u16>, ParseError> {
    if part.is_empty() {
        return Ok(Vec::new());
    }

    let mut groups = Vec::new();

    for (i, group) in part.split(':').enumerate() {
        if group.is_empty() || group.len() > 4 {
            return Err(ParseError::InvalidGroup(offset + i));
        }

        match u16::from_str_radix(group, 16) {
            Ok(value) => groups.push(value),
            Err(_) => return Err(ParseError::InvalidGroup(offset + i)),
        }
    }

    Ok(groups)
}

pub fn parse_ipv6(input: &str) -> Outcome {
    if input.is_empty() {
        return Outcome::Err(ParseError::InvalidFormat);
    }

    let groups: Vec<u16>;

    if let Some((left, right)) = input.split_once("::") {
        if right.contains("::") {
            return Outcome::Err(ParseError::InvalidFormat);
        }

        let left_groups = match parse_groups(left, 0) {
            Ok(groups) => groups,
            Err(error) => return Outcome::Err(error),
        };

        let right_groups = match parse_groups(right, left_groups.len()) {
            Ok(groups) => groups,
            Err(error) => return Outcome::Err(error),
        };

        let total = left_groups.len() + right_groups.len();

        if total >= 8 {
            return Outcome::Err(ParseError::WrongGroupCount(total));
        }

        groups = left_groups
            .into_iter()
            .chain(std::iter::repeat_n(0u16, 8 - total))
            .chain(right_groups)
            .collect();
    } else {
        let parsed = match parse_groups(input, 0) {
            Ok(groups) => groups,
            Err(error) => return Outcome::Err(error),
        };

        if parsed.len() != 8 {
            return Outcome::Err(ParseError::WrongGroupCount(parsed.len()));
        }

        groups = parsed;
    }

    let groups: [u16; 8] = match groups.try_into() {
        Ok(groups) => groups,
        Err(_) => return Outcome::Err(ParseError::InvalidFormat),
    };

    Outcome::Ok(Ipv6 { groups })
}
