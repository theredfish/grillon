pub enum Operator {
    Is,
    IsNot,
    Contains,
    DoesNotContain,
    Matches,
    DoesNotMatch,
    IsLessThan,
    JsonPath,
    IsBetween,
}

pub struct NumberComparand<T> {
    pub left: T,
    pub right: Option<T>,
}

pub struct Expression<T> {
    pub operator: Operator,
    pub value: T,
}

macro_rules! operator {
    ($name:ident, $o:expr) => {
        pub fn $name<T>(value: T) -> Expression<T> {
            Expression {
                operator: $o,
                value,
            }
        }
    };
}

pub fn is_between<T>(min: T, max: T) -> Expression<NumberComparand<T>> {
    Expression {
        operator: Operator::IsBetween,
        value: NumberComparand {
            left: min,
            right: Some(max),
        },
    }
}

pub fn is_less_than<T>(number: T) -> Expression<NumberComparand<T>> {
    Expression {
        operator: Operator::IsLessThan,
        value: NumberComparand {
            left: number,
            right: None,
        },
    }
}

operator!(is, Operator::Is);
operator!(is_not, Operator::IsNot);
operator!(contains, Operator::Contains);
operator!(does_not_contain, Operator::DoesNotContain);
operator!(matches, Operator::Matches);
operator!(does_not_match, Operator::DoesNotMatch);
operator!(jsonpath, Operator::JsonPath);
