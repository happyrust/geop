use crate::{
    bernstein_basis::BernsteinBasis, efloat::EFloat64, monomial_polynom::MonomialPolynom, HasZero,
    MultiDimensionFunction, OneDimensionFunction, ToMonomialPolynom,
};

pub struct BernsteinPolynomial<T> {
    coefficients: Vec<T>,
}

impl<T> BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    pub fn new(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    // pub fn from_monomail_polynom(monomial_polynom: MonomialPolynom) -> Self {
    //     // let mut coefficients = Vec::new();
    //     // for i in 0..=monomial_polynom.degree() {
    //     //     let basis = BernsteinBasis::new(i, monomial_polynom.degree()).unwrap();
    //     //     let basis_monomial = basis.to_monomial_polynom();
    //     //     let coeff = monomial_polynom.coefficient(&basis_monomial);
    //     //     coefficients.push(coeff);
    //     // }

    //     // Self { coefficients }
    // }

    pub fn to_monomial_polynom(&self) -> MonomialPolynom {
        let mut result = MonomialPolynom::zero();
        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BernsteinBasis::new(i, self.coefficients.len() - 1).unwrap();
            let basis_monomial = basis.to_monomial_polynom();
            let coeff = coeff.to_monomial_polynom();
            let term = &coeff * &basis_monomial;
            result = &result + &term;
        }

        result
    }
}

impl<T> MultiDimensionFunction<T> for BernsteinPolynomial<T>
where
    T: Clone,
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<EFloat64, Output = T>,
    T: HasZero,
    T: ToMonomialPolynom,
{
    fn eval(&self, t: EFloat64) -> T {
        let mut result = T::zero();

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let basis = BernsteinBasis::new(i, self.coefficients.len() - 1).unwrap();
            let basis_eval = basis.eval(t);
            result = result + coeff.clone() * basis_eval;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bernstein_polynomial() {
        let coeffs = vec![
            EFloat64::from(1.0),
            EFloat64::from(2.0),
            EFloat64::from(1.0),
            EFloat64::from(5.0),
            EFloat64::from(3.0),
        ];
        let b = BernsteinPolynomial::new(coeffs);
        let as_mon = b.to_monomial_polynom();
        for t in [0.15, 0.2, 0.67, 0.43456, 0.6373] {
            let eval = b.eval(EFloat64::from(t));
            println!("Eval at {}: {}", t, eval);
            assert_eq!(eval, as_mon.eval(EFloat64::from(t)));
        }

        println!("{}", &as_mon);
    }
}
