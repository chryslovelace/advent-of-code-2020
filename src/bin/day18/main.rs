use lazy_static::lazy_static;

peg::parser! {
    grammar arithmetic() for str {
        rule _() = [' ' | '\n']*

        pub rule expression1() -> i64 = precedence! {
            l:(@) "*" r:@ { l * r }
            l:(@) "+" r:@ { l + r }
            --
            _ n:literal() _ { n }
            _ "(" _ e:expression1() _ ")" _ { e }
        }

        pub rule expression2() -> i64 = precedence! {
            l:(@) "*" r:@ { l * r }
            --
            l:(@) "+" r:@ { l + r }
            --
            _ n:literal() _ { n }
            _ "(" _ e:expression2() _ ")" _ { e }
        }

        rule literal() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }
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
            .map(|expr| arithmetic::expression1(expr).unwrap())
            .sum::<i64>()
    );
}

fn part2() {
    println!(
        "**  {}",
        HOMEWORK
            .iter()
            .map(|expr| arithmetic::expression2(expr).unwrap())
            .sum::<i64>()
    );
}

fn main() {
    part1();
    part2();
}
