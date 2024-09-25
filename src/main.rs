use std::fmt::Debug;

fn main() {
    let fraction = fraction_calculation("1/3 + 2/6 × 3/5");
    println!("{:?}", fraction)
}

fn fraction_calculation(formula: &str) -> Fraction {
    let mut formula: Formula = to_formula(formula);

    print_formula(&formula);

    while let Some(i) = formula.iter().position(|item| match item {
        Item::Fraction(_) => false,
        Item::Symbol(s) => match s {
            Symbol::Times | Symbol::Divided => true,
            _ => false,
        },
    }) {
        let fraction = calculation(
            formula[i].symbol(),
            formula[i - 1].fraction(),
            formula[i + 1].fraction(),
        );

        for _ in 0..2 {
            formula.remove(i - 1);
        }
        formula[i - 1] = Item::Fraction(fraction);
        print_formula(&formula);
    }

    while formula.len() != 1 {
        let fraction = calculation(
            formula[1].symbol(),
            formula[0].fraction(),
            formula[2].fraction(),
        );

        for _ in 0..2 {
            formula.remove(0);
        }
        formula[0] = Item::Fraction(fraction);

        print_formula(&formula);
    }

    let result = formula[0].fraction();
    let numerator = result.numerator;
    let denominator = result.denominator;

    let divide = reduction(numerator, denominator);

    Fraction::new(numerator / divide, denominator / divide)
}

/// 計算
fn calculation(symbol: Symbol, f1: Fraction, f2: Fraction) -> Fraction {
    match symbol {
        Symbol::Plus => {
            let (f1_m, f2_m, d) = common_multiple(f1.denominator, f2.denominator);
            Fraction::new((f1.numerator * f1_m) + (f2.numerator * f2_m), d)
        }
        Symbol::Minus => {
            let (f1_m, f2_m, d) = common_multiple(f1.denominator, f1.denominator);
            Fraction::new((f1.numerator * f1_m) - (f2.numerator * f2_m), d)
        }
        Symbol::Times => {
            Fraction::new(f1.numerator * f2.numerator, f1.denominator * f2.denominator)
        }
        Symbol::Divided => {
            Fraction::new(f1.numerator * f2.denominator, f1.denominator * f2.numerator)
        }
    }
}

// 公倍数
fn common_multiple(num1: i64, num2: i64) -> (i64, i64, i64) {
    let mut n = 1;

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    loop {
        v1.push(num1 * n);
        v2.push(num2 * n);

        for (i, num) in v1.iter().enumerate() {
            if let Some(i2) = v2.iter().position(|n| n == num) {
                return (i as i64 + 1, i2 as i64 + 1, *num);
            }
        }

        n += 1;
    }
}

/// 公約数
fn reduction(num1: i64, num2: i64) -> i64 {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for i in 1..num1 {
        if num1 % i == 0 {
            v1.push(i);
        }
    }
    for i in 1..num2 {
        if num2 % i == 0 {
            v2.push(i);
        }
    }

    for num in v1.iter().rev() {
        if let Some(result) = v2.iter().find(|&x| x == num) {
            return *result;
        }
    }

    1
}

/// 文字列を式に変換
fn to_formula(str: &str) -> Formula {
    let formula = str.split(' ').map(|str| str.into()).collect();
    if !check(&formula) {
        panic!()
    }
    formula
}

/// 式が成り立つか判定
fn check(formula: &Formula) -> bool {
    if formula.len() % 2 == 0 {
        return false;
    }

    for (i, item) in formula.iter().enumerate() {
        match *item {
            Item::Fraction(_) => {
                if i % 2 != 0 {
                    return false;
                }
            }
            Item::Symbol(_) => {
                if i % 2 == 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn print_formula(formula: &Formula) {
    formula.iter().for_each(|item| print!("{:?} ", item));
    println!()
}

/// 式
type Formula = Vec<Item>;

enum Item {
    Symbol(Symbol),
    Fraction(Fraction),
}

/// 記号
#[derive(Clone, Copy)]
enum Symbol {
    Plus,
    Minus,
    Times,
    Divided,
}

/// 分数
#[derive(Clone, Copy)]
struct Fraction {
    /// 分子
    numerator: i64,
    /// 分母
    denominator: i64,
}

/// 式をItemに変換
const fn fraction(numerator: i64, denominator: i64) -> Item {
    Item::Fraction(Fraction::new(numerator, denominator))
}

const PLUS: Item = Item::Symbol(Symbol::Plus);
const MINUS: Item = Item::Symbol(Symbol::Minus);
const TIMES: Item = Item::Symbol(Symbol::Times);
const DIVIDED: Item = Item::Symbol(Symbol::Divided);

impl Item {
    const fn fraction(&self) -> Fraction {
        match self {
            Item::Fraction(f) => *f,
            Item::Symbol(_) => panic!(),
        }
    }
    const fn symbol(&self) -> Symbol {
        match self {
            Item::Fraction(_) => panic!(),
            Item::Symbol(s) => *s,
        }
    }
}

impl Fraction {
    const fn new(numerator: i64, denominator: i64) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl From<&str> for Item {
    fn from(str: &str) -> Self {
        match str {
            "+" => PLUS,
            "-" => MINUS,
            "×" => TIMES,
            "÷" => DIVIDED,
            _ => {
                let nums: Vec<i64> = str.split('/').map(|s| s.trim().parse().unwrap()).collect();
                fraction(nums[0], nums[1])
            }
        }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::Plus => "+",
                Symbol::Minus => "-",
                Symbol::Times => "×",
                Symbol::Divided => "÷",
            }
        )
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Fraction(fraction) => write!(f, "{:?}", fraction),
            Item::Symbol(symbol) => write!(f, "{:?}", symbol),
        }
    }
}
