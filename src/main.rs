use std::{
    fmt::Display,
    ops::{Add, Div, Index, Mul, Neg, Sub},
};

fn main() {
    let a = Polynom {
        coefficient: vec![-1., 0., 1.],
    };
    let b = Polynom {
        coefficient: vec![-1., 1.],
    };
    println!(
        "a: {}",
        a
    );
    println!(
        "b: {}",
        b
    );
    println!(
        "a+b: {}",
        &a + &b
    );
    println!(
        "a*b: {}",
        &a * &b
    );

    println!(
        "a/b:  {}",
        &a / &b
    );
}

#[derive(Debug, Clone)]
struct Polynom {
    coefficient: Vec<f32>,
}
impl Index<usize> for Polynom {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.coefficient.get(index).unwrap_or(&0.)
    }
}
impl Add for &Polynom {
    type Output = Polynom;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coefficient: Vec<_> = (0..self.coefficient.len().max(rhs.coefficient.len()))
            .map(
                |i| {
                    self.coefficient.get(i).copied().unwrap_or(0.)
                        + rhs.coefficient.get(i).copied().unwrap_or(0.)
                },
            )
            .rev()
            .skip_while(|x| *x == 0.)
            .collect();

        coefficient.reverse();
        Polynom { coefficient }
    }
}
impl Mul for &Polynom {
    type Output = Polynom;

    fn mul(self, rhs: Self) -> Self::Output {
        let coefficient = (0..(self.coefficient.len() + rhs.coefficient.len() - 1))
            .map(|i| (0..=i).map(|k| self[k] * rhs[i - k]).sum())
            .collect();
        Polynom { coefficient }
    }
}
impl Neg for Polynom {
    type Output = Polynom;

    fn neg(mut self) -> Self::Output {
        for x in &mut self.coefficient {
            *x = -*x;
        }
        self
    }
}
impl Sub for &Polynom {
    type Output = Polynom;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &-rhs.clone()
    }
}
impl Display for Polynom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coefficient.is_empty() {
            return write!(
                f,
                "0"
            );
        }
        let mut first = true;
        for (x, &m) in self.coefficient.iter().enumerate().rev() {
            if m == 0. {
                continue;
            }
            if first {
                first = false;
            } else {
                write!(
                    f,
                    " + "
                )?;
            }
            if m != 1.0 || x == 0 {
                write!(
                    f,
                    "{m}"
                )?;
            }
            match x {
                0 => {}
                1 => write!(
                    f,
                    "x"
                )?,
                _ => write!(
                    f,
                    "x^{}",
                    x
                )?,
            }
        }
        Ok(())
    }
}
impl Div for &Polynom {
    type Output = Polynom;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.coefficient.is_empty() {
            panic!("Division by 0 Polynom");
        }
        let len = (self.coefficient.len() + 1).saturating_sub(rhs.coefficient.len());
        let mut res = Polynom {
            coefficient: vec![0.; len],
        };
        for i in (0..len).rev() {
            let rest = self - &(&res * rhs);
            *res.coefficient.get_mut(i).unwrap() =
                rest.coefficient.last().unwrap_or(&0.) / rhs.coefficient.last().unwrap();
        }
        res
    }
}
