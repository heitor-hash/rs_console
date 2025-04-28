use rand::{rngs::ThreadRng, Rng};
use std::io::{self};

fn rng_game(mut rng: ThreadRng) {
    let mut lo_str: String = String::new();
    let mut hi_str: String = String::new();
    println!("Enter the range lower part");
    io::stdin()
        .read_line(&mut lo_str)
        .expect("Error reading line");
    let lo: u16;
    match lo_str.trim().parse::<u16>() {
        Ok(num) => {
            lo = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }
    println!("Enter the range higher part");
    io::stdin()
        .read_line(&mut hi_str)
        .expect("Error reading line");
    let hi: u16;
    match hi_str.trim().parse::<u16>() {
        Ok(num) => {
            hi = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    if lo > hi {
        println!("Error low is bigger than high");
        return;
    }
    let randnum: u16 = rng.random_range(lo..=hi);

    println!("You can quit by typing: back");
    println!("Try to find it by typing a random number");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro");
        if input == "back" {
            break;
        }
        let input = input.trim().parse::<u16>().unwrap();
        if input == randnum {
            println!("You win");
            break;
        } else if input > randnum {
            println!("Your number is bigger than the random number");
        } else {
            println!("Your number is smaller than the random number");
        }
    }
}

fn rng_number_range(mut rng: ThreadRng) {
    let mut lo_str: String = String::new();
    let mut hi_str: String = String::new();
    println!("Enter the range lower part");
    io::stdin()
        .read_line(&mut lo_str)
        .expect("Error reading line");
    let lo: u16;
    match lo_str.trim().parse::<u16>() {
        Ok(num) => {
            lo = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }
    println!("Enter the range higher part");
    io::stdin()
        .read_line(&mut hi_str)
        .expect("Error reading line");
    let hi: u16;
    match hi_str.trim().parse::<u16>() {
        Ok(num) => {
            hi = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    if lo > hi {
        println!("Error low is bigger than high");
        return;
    }

    let rand1: u16 = rng.random_range(lo..=hi);
    let rand2: u16 = rng.random_range(lo..=hi);
    let rand3: u16 = rng.random_range(lo..=hi);

    println!("random number 1 is {rand1}");
    println!("random number 2 is {rand2}");
    println!("random number 3 is {rand3}");
}

fn main_rng() -> bool {
    let rng = rand::rng();

    loop {
        let mut input = String::new();
        println!("RNG");
        println!("Enter: Game, Number, Back or Exit(turn-off program)");
        io::stdin().read_line(&mut input).expect("Erro");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "game" => rng_game(rng.clone()),
            "number" => rng_number_range(rng.clone()),
            "back" => return false,
            "exit" => return true,
            _ => {
                println!("Comando invalido");
                continue;
            }
        }
    }
}

fn main_calculator() {
    println!("Digite um numero: ");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");
    let input1: f64;
    match input.trim().parse::<f64>() {
        Ok(num) => {
            input1 = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    println!("Digite um operador:");
    println!("1: adição");
    println!("2: subtração");
    println!("3: multiplicação");
    println!("4: divisão");

    let mut operador: String = String::new();
    io::stdin()
        .read_line(&mut operador)
        .expect("Failed to read operator");
    let operador = operador.trim().parse::<u8>().unwrap();
    match operador {
        0 => {
            // error, return
            println!("Please enter a valid operator");
            return;
        }
        1..=4 => (),
        // from 5 to above, error, return
        _ => {
            println!("Please enter a valid operator");
            return;
        }
    }

    // Para editar no futuro,
    // aqui será para operações
    // com um unico número

    println!("Digite outro número");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");
    let input2: f64;
    match input.trim().parse::<f64>() {
        Ok(num) => {
            input2 = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    // Aqui será para operações
    // com dois números

    match operador {
        1 => {
            let res: f64 = input1 + input2;
            println!("Resposta: {res}")
        }
        2 => {
            let res: f64 = input1 - input2;
            println!("Resposta: {res}")
        }
        3 => {
            let res: f64 = input1 * input2;
            println!("Resposta: {res}")
        }
        4 => {
            if input2 != 0.0 {
                let res: f64 = input1 / input2;
                println!("Resposta: {res}")
            } else {
                println!("Erro divisão por 0")
            }
        }
        _ => (),
    }
    // wait a bit to let the user see the answear:
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn printer() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Erro");
    println!("You typed: {input}");
    println!("do you want to continue? [Y/N]");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "y" => printer(),
        "n" => return,
        _ => {
            println!("? Vou sair do print");
            return;
        }
    }
}

fn main() {
    println!("Hello, world!");

    // main loop:
    loop {
        // What do you want? enter: Calculator, Print or Exit
        println!("Enter: Calculator, Print, Rng or Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "calculator" => main_calculator(),
            "print" => printer(),
            "rng" => {
                let quitter: bool = main_rng();
                if quitter {
                    break;
                }
            }
            "exit" => break,
            _ => println!("Please enter a valid command"),
        }
    }
    // end main loop
}
