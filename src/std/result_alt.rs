mod checked {
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        NonPositiveLogarithm,
    }
    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        }  else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Intermediate function

    fn op_(x: f64, y: f64) -> MathResult {
        // If `div` fails, then `DivisionByZero` will be returned
        let ratio = div(x, y)?;
        // If `ln` fails, then `NonPositiveLogarithm` will be returned
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Ok(value) => println!("{}", value),
            Err(why) => panic!("{}", match why {
                MathError::DivisionByZero => "Division by zero",
                MathError::NegativeSquareRoot => "Square root of negative number",
                MathError::NonPositiveLogarithm => "Non positive logarithm",
            })
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}