use std::io::{stdout, stdin, Write};

fn determinant(coxsq: &f64, cox: &f64, cons: &f64) -> f64 {
    /*
     * determinant of a quadratic eq is cox^2 - 4*coxsq*const
     *
     * finding the determinant is quite important because with
     * this value, one can determine if the roots are natural
     * or imaginary.
     */

    cox.powi(2) - (4.0 * coxsq * cons)
}

fn get_coeff() -> (f64, f64, f64) {
    /*
     * get the coefficents from stdin
     *
     * returns a tuple of coefficients and constants
     * the order goes by this
     * (coefficient of x^2, coefficient of x, constant)
     */

    // closure to print string
    let printstr = |string: &str| {
        print!("{}", string);

        stdout()
            .flush()
            .unwrap();
    };

    // closure to get input. specifically returns f64
    let input = |prompt: &str| {
        let mut num = String::new();

        printstr(prompt);

        // read from stdin
        stdin()
            .read_line(&mut num)
            .expect("can't read from stdin");

        // convert string to f64
        let num: f64 = num
            .trim()
            .parse()
            .unwrap_or_else(|_| 0.0);

        num
    };

    // coefficient of x squared
    let coefxsq: f64 = input(&"enter the coefficient of x squared: ");

    if coefxsq == 0.0 {
        println!("This is not a quadratic equation!");
        std::process::exit(1);
    }

    // coefficient of x
    let coefx: f64 = input(&"enter the coefficient of x: ");
    // the constant value
    let cons: f64 = input(&"enter the constant: ");

    (coefxsq, coefx, cons)
}

pub fn calculate_root() -> (f64, f64, bool) {
    /*
     * calculate the root of the qudratic equation
     * get coefficients directly from get_coeff
     *
     * the returned values are root, root, is_real bool
     */

    let mut is_real: bool = true;

    // get the coefficients
    let coeffs = get_coeff();
    let cofxsq = coeffs.0;
    let cofx = coeffs.1;
    let cons = coeffs.2;

    println!(
        "the quadratic equation is {}x^2 + {}x + {}",
        cofxsq,
        cofx,
        cons
    );

    // calculate the determinant
    let determinant = determinant(&cofxsq, &cofx, &cons);

    // check if it real
    if determinant < 0.0 { is_real = false; }

    // take the abs value of determinant
    let determinant = determinant.abs();

    // calculate -b/2a
    let fraction: f64 = -cofx / (2.0 * cofxsq);

    (fraction+determinant.sqrt(), fraction-determinant.sqrt(), is_real)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_test() {
        println!("{}", determinant(&4.0, &7.0, &4.0));
    }

    //#[test]
    fn input_test() {
        get_coeff();
    }

    #[test]
    fn root_test() {
        let kek = calculate_root();
        println!("{:?}", kek);
    }
}
