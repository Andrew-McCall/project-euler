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
    let mut translation = Vec::new();

    let teens = n % 100;
    if teens > 10 && teens < 20 {
        match teens {
            10 => translation.push(NumberTranslation::Ten),
            11 => translation.push(NumberTranslation::Eleven),
            12 => translation.push(NumberTranslation::Twelve),
            13 => translation.push(NumberTranslation::Thirteen),
            14 => translation.push(NumberTranslation::Fourteen),
            15 => translation.push(NumberTranslation::Fifteen),
            16 => translation.push(NumberTranslation::Sixteen),
            17 => translation.push(NumberTranslation::Seventeen),
            18 => translation.push(NumberTranslation::Eighteen),
            19 => translation.push(NumberTranslation::Nineteen),
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
            1 => (),
            2 => match digit {
                1 => translation.push(NumberTranslation::Ten),
                2 => translation.push(NumberTranslation::Twenty),
                3 => translation.push(NumberTranslation::Thirty),
                4 => translation.push(NumberTranslation::Forty),
                5 => translation.push(NumberTranslation::Fifty),
                6 => translation.push(NumberTranslation::Sixty),
                7 => translation.push(NumberTranslation::Seventy),
                8 => translation.push(NumberTranslation::Eighty),
                9 => translation.push(NumberTranslation::Ninety),
                _ => unreachable!(),
            },
            3 => {
                if !translation.is_empty() {
                    translation.push(NumberTranslation::And)
                };
                translation.push(NumberTranslation::Hundred);
            }
            4 => {
                translation.push(NumberTranslation::Thousand);
            }
            5 => {
                translation.push(NumberTranslation::Thousand);
            }
            6 => {
                translation.push(NumberTranslation::Million);
            }
            7 => {
                translation.push(NumberTranslation::Million);
            }
            8 => {
                translation.push(NumberTranslation::Billion);
            }
            9 => {
                translation.push(NumberTranslation::Billion);
            }
            10 => {
                translation.push(NumberTranslation::Trillion);
            }
            11 => {
                translation.push(NumberTranslation::Trillion);
            }
            12 => {
                translation.push(NumberTranslation::Quadrillion);
            }
            13 => {
                translation.push(NumberTranslation::Quadrillion);
            }
            14 => {
                translation.push(NumberTranslation::Quintillion);
            }
            15 => {
                translation.push(NumberTranslation::Quintillion);
            }
            _ => panic!("Number too large"),
        }

        if column != 2
            && let Some(d) = match_digit(digit)
        {
            translation.push(d)
        }
    }

    let string = translation
        .iter()
        .rev()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    Result {
        characters: string.chars().filter(|c| char::is_alphabetic(*c)).count() as u64,
        number_string: string,
        number_value: n,
    }
}

pub fn match_digit(digit: u64) -> Option<NumberTranslation> {
    assert!(digit < 10);
    match digit {
        0 => None,
        1 => Some(NumberTranslation::One),
        2 => Some(NumberTranslation::Two),
        3 => Some(NumberTranslation::Three),
        4 => Some(NumberTranslation::Four),
        5 => Some(NumberTranslation::Five),
        6 => Some(NumberTranslation::Six),
        7 => Some(NumberTranslation::Seven),
        8 => Some(NumberTranslation::Eight),
        9 => Some(NumberTranslation::Nine),
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    assert_eq!(alt_solution(1000).number_string, "one thousand");
    assert_eq!(
        alt_solution(999).number_string,
        "nine hundred and ninety nine"
    );
    assert_eq!(alt_solution(600).number_string, "six hundred");
    assert_eq!(alt_solution(611).number_string, "six hundred and eleven");
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
