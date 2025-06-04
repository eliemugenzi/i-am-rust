mod checked {
    // Mathematical `errors` we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else  {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        }  else  {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        }  else  {
            Ok(x.ln())
        }
    }
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Ok(ratio) => match checked::ln(ratio) {
            Ok(ln) => match checked::sqrt(ln) {
                Ok(sqrt) => sqrt,
                Err(why) => panic!("{:?}", why),
            },
            Err(why) => panic!("{:?}", why),
        },
        Err(why) => panic!("{:?}", why),
    }
}

fn main() {
    // Will this fail?
    println!("{:?}", op(1.0, 10.0));
}