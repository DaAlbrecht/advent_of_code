fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
    with_conditionals: bool,
    mul_enabled: bool,
}
impl Iterator for Lexer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
impl Lexer {
    pub fn new(input: String, with_conditionals: bool) -> Lexer {
        let mut lexer = Lexer {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: 0,
            with_conditionals,
            mul_enabled: true,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Option<usize> {
        if self.ch == 0 {
            return None;
        }
        if self.with_conditionals {
            if self.ch == b'm' && self.mul_enabled {
                let produkt = self.read_mul();
                if produkt.is_none() {
                    self.read_char();
                    return Some(0);
                }
                return produkt;
            }
            self.read_conditionals();
            Some(0)
        } else {
            let produkt = self.read_mul();

            if produkt.is_none() {
                self.read_char();
                return Some(0);
            }
            produkt
        }
    }

    fn read_mul(&mut self) -> Option<usize> {
        if self.ch != b'm' {
            return None;
        }
        self.read_char();

        if self.ch != b'u' {
            return None;
        }

        self.read_char();
        if self.ch != b'l' {
            return None;
        }

        self.read_char();
        if self.ch != b'(' {
            return None;
        }

        self.read_char();
        if !self.ch.is_ascii_digit() {
            return None;
        }

        let mut position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let first_factor: String = self.input[position..self.position]
            .iter()
            .map(|&c| c as char)
            .collect();

        if self.ch != b',' {
            return None;
        }

        self.read_char();
        if !self.ch.is_ascii_digit() {
            return None;
        }
        position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let second_factor: String = self.input[position..self.position]
            .iter()
            .map(|&c| c as char)
            .collect();

        if self.ch != b')' {
            return None;
        }
        self.read_char();
        Some(first_factor.parse::<usize>().unwrap() * second_factor.parse::<usize>().unwrap())
    }

    fn read_conditionals(&mut self) {
        //don't()
        //do()
        if self.ch != b'd' {
            self.read_char();
            return;
        }
        self.read_char();
        if self.ch != b'o' {
            return;
        }
        match self.peek() {
            b'(' => {
                self.read_char();
                self.read_char();
                if self.ch != b')' {
                    return;
                }
                self.mul_enabled = true;
            }
            b'n' => {
                self.read_char();
                self.read_char();
                if self.ch != b'\'' {
                    return;
                }
                self.read_char();
                if self.ch != b't' {
                    return;
                }
                self.read_char();
                if self.ch != b'(' {
                    return;
                }
                self.read_char();
                if self.ch != b')' {
                    return;
                }
                self.mul_enabled = false;
            }
            _ => (),
        };
    }
}

fn part_01(input: &str) -> usize {
    let mut sum = 0;
    let mut lexer = Lexer::new(input.into(), false);
    while let Some(next) = lexer.next_token() {
        sum += next;
    }
    sum
}
fn part_02(input: &str) -> usize {
    let mut sum = 0;
    let mut lexer = Lexer::new(input.into(), true);
    while let Some(next) = lexer.next_token() {
        sum += next;
    }
    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(super::part_01(test_input), 161);
    }
    #[test]
    fn part_02() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::part_02(test_input), 48);
    }
}
