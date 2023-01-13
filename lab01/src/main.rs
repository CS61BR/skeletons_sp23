use std::io::{self, Write};

#[cfg(test)]
mod tests;

pub fn product(a: i64, b: i64) -> i64 {
    a * b
}

pub fn sum(a: i64, b: i64) -> i64 {
    a * b
}



fn main() -> io::Result<()> {
    let num1 = prompt("Give me an integer: ")?;
    let num2 = prompt("Give me another integer: ")?;

    println!("The product of {num1} and {num2} is {}", product(num1, num2));
    println!("The sum of {num1} and {num2} is {}", sum(num1, num2));
    Ok(())
}


fn prompt(message: &str) -> io::Result<i64> {
    loop {
        print!("{}", message);
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim().parse() {
            Ok(x) => return Ok(x),
            Err(_) => println!("That's not an integer!"),
        }
    }
}
