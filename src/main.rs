use std::env;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

enum TempUnit {
    Fahrenheit,
    Celsius,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseUnitError;

impl FromStr for TempUnit {
    type Err = ParseUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "fahrenheit" | "f" => Ok(TempUnit::Fahrenheit),
            "celsius" | "c" => Ok(TempUnit::Celsius),
            _ => Err(ParseUnitError),
        }
    }
}

impl Display for TempUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Celsius => write!(f, "Celsius"),
            Self::Fahrenheit => write!(f, "Fahrenheit"),
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let t: f64;
    let unit: TempUnit;
    if args.len() > 0 {
        t = args
            .next()
            .unwrap()
            .parse()
            .expect("Valid number as temperature supplied");
        unit = args
            .next()
            .unwrap()
            .parse()
            .expect("Supported units are either Fahrenheit, oder Celsius");
    } else {
        let mut s = String::new();
        println!("Enter the temperature to convert: ");
        io::stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        t = s
            .trim()
            .parse()
            .expect("Valid number as temperature supplied");
        println!("Enter temperatures unit (Fahrenheit or Celsius): ");
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        unit = s
            .trim()
            .parse()
            .expect("Supported units are either Fahrenheit, oder Celsius");
    }

    // °C equals °F minus 32, divided by 9/5
    let (t_converted, unit_converted, unit_input, unit_converted_enum) = match unit {
        TempUnit::Celsius => ((t * (9.0 / 5.0)) + 32.0, 'F', 'C', TempUnit::Fahrenheit),
        TempUnit::Fahrenheit => ((t - 32.0) / (9.0 / 5.0), 'C', 'F', TempUnit::Celsius),
    };

    println!("{t:.2} °{unit_input} = {t_converted:.2} °{unit_converted}\n");
    println!("{unit}: {t:.2} °{unit_input}");
    println!("{unit_converted_enum}: {t_converted:.2} °{unit_converted}");
}
