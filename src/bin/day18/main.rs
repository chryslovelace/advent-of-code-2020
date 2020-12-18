use lazy_static::lazy_static;

pub enum Expr {
    Literal(i64),
    Sum(Box<Expr>, Box<Expr>),
    Product(Box<Expr>, Box<Expr>),
}

peg::parser! {
    grammar arithmetic() for str {
        rule _() = [' ' | '\n']*

        pub rule expression1() -> Expr = precedence! {
            l:(@) "*" r:@ { Expr::Product(Box::new(l), Box::new(r)) }
            l:(@) "+" r:@ { Expr::Sum(Box::new(l), Box::new(r)) }
            --
            _ n:literal() _ { n }
            _ "(" _ e:expression1() _ ")" _ { e }
        }

        pub rule expression2() -> Expr = precedence! {
            l:(@) "*" r:@ { Expr::Product(Box::new(l), Box::new(r)) }
            --
            l:(@) "+" r:@ { Expr::Sum(Box::new(l), Box::new(r)) }
            --
            _ n:literal() _ { n }
            _ "(" _ e:expression2() _ ")" _ { e }
        }

        rule literal() -> Expr
            = n:$(['0'..='9']+) { Expr::Literal(n.parse().unwrap()) }
    }
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Literal(n) => *n,
            Expr::Sum(l, r) => l.eval() + r.eval(),
            Expr::Product(l, r) => l.eval() * r.eval(),
        }
    }
}

lazy_static! {
    static ref HOMEWORK: Vec<&'static str> = include_str!("input.txt").lines().collect();
}

fn part1() {
    println!(
        "*   {}",
        HOMEWORK
            .iter()
            .map(|expr| arithmetic::expression1(expr).unwrap().eval())
            .sum::<i64>()
    );
}

fn part2() {
    println!(
        "**  {}",
        HOMEWORK
            .iter()
            .map(|expr| arithmetic::expression2(expr).unwrap().eval())
            .sum::<i64>()
    );
}

fn main() {
    part1();
    part2();
}
