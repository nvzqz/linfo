//! License expressions.

use core::{
    convert::{TryFrom, TryInto},
    fmt,
};
use crate::{License, SpdxLicense, ParseError};

/// A license expression.
///
/// Grammar (note the padded spacing):
///
/// ```txt
/// Single = License
///
/// Or = License " OR " License
///    | License " OR " Or
///
/// And = License " AND " License
///     | License " AND " And
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    /// Only one license's conditions to follow.
    Single(License),
    /// The conditions of either of two or more licenses apply.
    Or(Or),
    /// The conditions of all of two or more licenses apply.
    And(And),
}

impl From<SpdxLicense> for Expr {
    #[inline]
    fn from(l: SpdxLicense) -> Self {
        Self::Single(l.into())
    }
}

impl From<License> for Expr {
    #[inline]
    fn from(l: License) -> Self {
        Self::Single(l)
    }
}

impl From<Or> for Expr {
    #[inline]
    fn from(or: Or) -> Self {
        Self::Or(or)
    }
}

impl From<And> for Expr {
    #[inline]
    fn from(and: And) -> Self {
        Self::And(and)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Single(ref l) => l.fmt(f),
            Expr::Or(ref or) => or.fmt(f),
            Expr::And(ref and) => and.fmt(f),
        }
    }
}

// TODO: Implement `TryFrom<&[u8]>` for `Expr`
impl<'a> TryFrom<&'a str> for Expr {
    type Error = ParseError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use core::{iter, str::Split};

        fn handle_iter<'a>(
            a: &'a str,
            b: &'a str,
            iter: Split<'a, &str>,
        ) -> Vec<License> {
            iter::once(a)
                .chain(iter::once(b))
                .chain(iter)
                .map(str::trim)
                .map(License::try_from)
                .flatten()
                .collect()
        }

        let s = s.trim();
        let mut iter_or = s.split(" OR ");

        match (iter_or.next(), iter_or.next()) {
            (None, _) => {
                Err(ParseError::Empty)
            },
            (Some(s), None) => {
                let mut iter_and = s.split(" AND ");

                match (iter_and.next(), iter_and.next()) {
                    (None, _) => {
                        Err(ParseError::Empty)
                    },
                    (Some(s), None) => {
                        s.try_into().map(Self::Single)
                    },
                    (Some(l1), Some(l2)) => {
                        Ok(And(handle_iter(l1, l2, iter_and)).into())
                    },
                }
            },
            (Some(l1), Some(l2)) => {
                Ok(Or(handle_iter(l1, l2, iter_or)).into())
            },
        }
    }
}

impl PartialEq<License> for Expr {
    #[inline]
    fn eq(&self, l: &License) -> bool {
        if let Expr::Single(e) = self {
            e == l
        } else {
            false
        }
    }
}

impl PartialEq<str> for Expr {
    fn eq(&self, s: &str) -> bool {
        let (list, sep) = match self {
            Expr::Single(l) => return l.id() == s,
            Expr::Or(or) => {
                (or.as_slice(), " OR ")
            },
            Expr::And(and) => {
                (and.as_slice(), " AND ")
            },
        };
        let mut iter = s.trim().split(sep).map(str::trim);
        for license in list {
            match iter.next() {
                None => return false,
                Some(next) if next != license.id() => return false,
                _ => {},
            }
        }
        iter.next().is_none()
    }
}

impl PartialEq<Expr> for str {
    #[inline]
    fn eq(&self, e: &Expr) -> bool {
        e == self
    }
}

impl Expr {
    /// Attempts to parse `input` and returns a
    /// [`ParseError`](struct.ParseError.html) on error.
    #[inline]
    pub fn parse<'i, I>(input: I) -> Result<Self, ParseError<'i>>
        where I: TryInto<Self, Error = ParseError<'i>> + 'i
    {
        input.try_into()
    }

    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License] {
        match self {
            Expr::Single(l) => core::slice::from_ref(l),
            Expr::Or(or) => or.as_slice(),
            Expr::And(and) => and.as_slice(),
        }
    }
}

/// A set of licenses separated by `OR`. For projects that are dual/n-ary
/// licensed.
///
/// See [`Expr::Or`](enum.Expr.html#variant.Or).
///
/// Instances of this type always have two or more.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Or(Vec<License>);

impl Or {
    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License] {
        self.0.as_slice()
    }
}

/// A set of licenses separated by `AND`. For projects that require restrictions
/// of multiple licenses.
///
/// See [`Expr::And`](enum.Expr.html#variant.And).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct And(Vec<License>);

impl And {
    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License] {
        self.0.as_slice()
    }
}

mod impl_display {
    use super::*;

    type Iter<'a> = core::slice::Iter<'a, License>;

    fn display_iter(mut iter: Iter, sep: &str, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(first) = iter.next() {
            fmt::Display::fmt(&first, f)?;
        } else {
            return Ok(());
        }
        for next in iter {
            write!(f, "{}{}", sep, next)?;
        }
        Ok(())
    }

    impl fmt::Display for Or {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            display_iter(self.0.iter(), " OR ", f)
        }
    }

    impl fmt::Display for And {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            display_iter(self.0.iter(), " AND ", f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_eq() {
        let licenses = [SpdxLicense::Mit, SpdxLicense::Apache2];
        let licenses = licenses.iter().map(|&l| License::from(l));

        for l1 in licenses.clone() {
            let expr = l1.id();
            let e = Expr::parse(expr).unwrap();
            assert_eq!(&e, expr);
            assert_eq!(e, Expr::Single(l1.clone()));

            for l2 in licenses.clone() {
                let exprs = [
                    format!("{}  OR  {}", l1, l2),
                    format!(" {} OR {} ", l1, l2),
                    format!(" {} OR {} OR {}", l1, l2, l1),
                    format!("{}  AND  {}", l1, l2),
                    format!(" {} AND {} ", l1, l2),
                    format!(" {} AND {} AND {}", l1, l2, l1),
                ];
                for expr in exprs.iter() {
                    let e = Expr::parse(expr.as_str()).unwrap();
                    assert_eq!(&e, expr.as_str());
                }
            }
        }
    }
}
