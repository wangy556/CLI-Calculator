use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    println!("Command Line Calculator (Type 'exit' to quit)");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().eq_ignore_ascii_case("exit") {
                    break;
                }
                match evaluate_expression(&line) {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err("Invalid expression. Format: <number> <operator> <number>".to_string());
    }

    let num1 = match tokens[0].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number format for the first operand.".to_string()),
    };

    let operator = tokens[1];
    let num2 = match tokens[2].parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid number format for the second operand.".to_string()),
    };

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                return Err("Division by zero is not allowed.".to_string());
            }
            num1 / num2
        }
        _ => return Err("Invalid operator. Supported operators: +, -, *, /".to_string()),
    };

    Ok(result)
}
