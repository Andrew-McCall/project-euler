/* p17: Number Letter Counts */
pub fn solution(n: u64) -> u64 {
    let mut sum = 0;

    for i in 1..=n {
        sum += alt_solution(i).characters;
    }

    sum
}

pub fn alt_solution(mut n: u64) -> Result {
    if n == 0 {
        return Result {
            characters: 5,
            number_string: "zero".to_string(),
            number_value: 0,
        };
    }

    let mut column = 0;
    let mut string = String::new();

    let teens = n % 100;
    if teens > 10 && teens < 20 {
        match teens {
            10 => string.push_str(&NumberTranslation::Ten.to_string()),
            11 => string.push_str(&NumberTranslation::Eleven.to_string()),
            12 => string.push_str(&NumberTranslation::Twelve.to_string()),
            13 => string.push_str(&NumberTranslation::Thirteen.to_string()),
            14 => string.push_str(&NumberTranslation::Fourteen.to_string()),
            15 => string.push_str(&NumberTranslation::Fifteen.to_string()),
            16 => string.push_str(&NumberTranslation::Sixteen.to_string()),
            17 => string.push_str(&NumberTranslation::Seventeen.to_string()),
            18 => string.push_str(&NumberTranslation::Eighteen.to_string()),
            19 => string.push_str(&NumberTranslation::Nineteen.to_string()),
            _ => unreachable!(),
        }
        column = 2;
        n /= 100;
    }

    while n > 0 {
        let digit = n % 10;
        n /= 10;
        column += 1;

        if digit == 0 && column > 1 {
            continue;
        }

        match column {
            1 => string.push_str(&match_digit(digit)),
            2 => match digit {
                1 => string.push_str(&NumberTranslation::Ten.to_string()),
                2 => string.push_str(&NumberTranslation::Twenty.to_string()),
                3 => string.push_str(&NumberTranslation::Thirty.to_string()),
                4 => string.push_str(&NumberTranslation::Forty.to_string()),
                5 => string.push_str(&NumberTranslation::Fifty.to_string()),
                6 => string.push_str(&NumberTranslation::Sixty.to_string()),
                7 => string.push_str(&NumberTranslation::Seventy.to_string()),
                8 => string.push_str(&NumberTranslation::Eighty.to_string()),
                9 => string.push_str(&NumberTranslation::Ninety.to_string()),
                _ => unreachable!(),
            },
            3 => {
                if !string.is_empty() {
                    string.push_str(&NumberTranslation::And.to_string())
                };
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Hundred.to_string());
            }
            4 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Thousand.to_string());
            }
            5 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Thousand.to_string());
            }
            6 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Million.to_string());
            }
            7 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Million.to_string());
            }
            8 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Billion.to_string());
            }
            9 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Billion.to_string());
            }
            10 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Trillion.to_string());
            }
            11 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Trillion.to_string());
            }
            12 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Quadrillion.to_string());
            }
            13 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Quadrillion.to_string());
            }
            14 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Quintillion.to_string());
            }
            15 => {
                string.push_str(&match_digit(digit));
                string.push_str(&NumberTranslation::Quintillion.to_string());
            }
            _ => panic!("Number too large"),
        }
    }

    Result {
        characters: string.chars().filter(|c| char::is_alphabetic(*c)).count() as u64,
        number_string: string,
        number_value: n,
    }
}

pub fn match_digit(digit: u64) -> String {
    assert!(digit < 10);
    match digit {
        0 => "".to_string(),
        1 => NumberTranslation::One.to_string(),
        2 => NumberTranslation::Two.to_string(),
        3 => NumberTranslation::Three.to_string(),
        4 => NumberTranslation::Four.to_string(),
        5 => NumberTranslation::Five.to_string(),
        6 => NumberTranslation::Six.to_string(),
        7 => NumberTranslation::Seven.to_string(),
        8 => NumberTranslation::Eight.to_string(),
        9 => NumberTranslation::Nine.to_string(),
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    assert_eq!(alt_solution(1000).number_string, "onethousand");
    assert_eq!(alt_solution(999).number_string, "nineninetyandninehundred");
    assert_eq!(alt_solution(600).number_string, "sixhundred");
    assert_eq!(alt_solution(611).number_string, "elevenandsixhundred");
    assert_eq!(solution(1000), 21124);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Result {
    characters: u64,
    number_string: String,
    number_value: u64,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = '{}' ({})",
            self.number_value, self.number_string, self.characters
        )
    }
}

pub enum NumberTranslation {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    Hundred,
    Thousand,
    Million,
    Billion,
    Trillion,
    Quadrillion,
    Quintillion,
    And,
}

impl NumberTranslation {
    pub fn count(&self) -> u64 {
        self.to_string().len() as u64
    }
}

impl std::fmt::Display for NumberTranslation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NumberTranslation::Zero => "zero",
            NumberTranslation::One => "one",
            NumberTranslation::Two => "two",
            NumberTranslation::Three => "three",
            NumberTranslation::Four => "four",
            NumberTranslation::Five => "five",
            NumberTranslation::Six => "six",
            NumberTranslation::Seven => "seven",
            NumberTranslation::Eight => "eight",
            NumberTranslation::Nine => "nine",
            NumberTranslation::Ten => "ten",
            NumberTranslation::Eleven => "eleven",
            NumberTranslation::Twelve => "twelve",
            NumberTranslation::Thirteen => "thirteen",
            NumberTranslation::Fourteen => "fourteen",
            NumberTranslation::Fifteen => "fifteen",
            NumberTranslation::Sixteen => "sixteen",
            NumberTranslation::Seventeen => "seventeen",
            NumberTranslation::Eighteen => "eighteen",
            NumberTranslation::Nineteen => "nineteen",
            NumberTranslation::Twenty => "twenty",
            NumberTranslation::Thirty => "thirty",
            NumberTranslation::Forty => "forty",
            NumberTranslation::Fifty => "fifty",
            NumberTranslation::Sixty => "sixty",
            NumberTranslation::Seventy => "seventy",
            NumberTranslation::Eighty => "eighty",
            NumberTranslation::Ninety => "ninety",
            NumberTranslation::Hundred => "hundred",
            NumberTranslation::Thousand => "thousand",
            NumberTranslation::Million => "million",
            NumberTranslation::Billion => "billion",
            NumberTranslation::Trillion => "trillion",
            NumberTranslation::Quadrillion => "quadrillion",
            NumberTranslation::Quintillion => "quintillion",
            NumberTranslation::And => "and",
        };
        write!(f, "{}", s)
    }
}
