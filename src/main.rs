use rand::{rngs::ThreadRng, Rng};
use std::io;

mod mod1;
mod mod2;

fn rng_game(mut rng: ThreadRng) {
    let mut lo_str: String = String::new();
    let mut hi_str: String = String::new();
    println!("Digite o minimo");
    io::stdin()
        .read_line(&mut lo_str)
        .expect("Error reading line");
    let lo: u32;
    match lo_str.trim().parse::<u32>() {
        Ok(num) => {
            lo = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }
    println!("Digite o máximo");
    io::stdin()
        .read_line(&mut hi_str)
        .expect("Error reading line");
    let hi: u32;
    match hi_str.trim().parse::<u32>() {
        Ok(num) => {
            hi = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    if lo > hi {
        println!("Erro minimo maior que maximo");
        return;
    }
    let randnum: u32 = rng.random_range(lo..=hi);

    println!("Você pode sair digitando: back");
    println!("Digite um número aleatorio");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro, Digite um número válido");
        if input == "back" {
            break;
        }
        let input = input.trim().parse::<u32>().unwrap();
        if input == randnum {
            println!("Você ganhou");
            break;
        } else if input > randnum {
            println!("Seu número é maior que o número aleatório");
        } else {
            println!("Seu número é menor que o número aleatório");
        }
        println!("Digite outro número");
    }
}

fn rng_number_range(mut rng: ThreadRng) {
    let mut lo_str: String = String::new();
    let mut hi_str: String = String::new();
    println!("Digite o minimo");
    io::stdin()
        .read_line(&mut lo_str)
        .expect("Error reading line");
    let lo: u32;
    match lo_str.trim().parse::<u32>() {
        Ok(num) => {
            lo = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }
    println!("Digite o máximo");
    io::stdin()
        .read_line(&mut hi_str)
        .expect("Error reading line");
    let hi: u32;
    match hi_str.trim().parse::<u32>() {
        Ok(num) => {
            hi = num;
        }
        Err(err) => {
            println!("Erro: {err}");
            return;
        }
    }

    if lo > hi {
        println!("Erro minimo é maior que máximo");
        return;
    }

    let rand1: u32 = rng.random_range(lo..=hi);
    let rand2: u32 = rng.random_range(lo..=hi);
    let rand3: u32 = rng.random_range(lo..=hi);

    println!("numero random 1 é: {rand1}");
    println!("numero random 2 é: {rand2}");
    println!("numero random 3 é: {rand3}");
}

fn main_rng() -> bool {
    let rng: ThreadRng = rand::rng();

    loop {
        let mut input: String = String::new();
        println!("RNG");
        println!("Digite: Jogo, Numero, Back ou Exit(Sai do programa)");
        io::stdin().read_line(&mut input).expect("Erro");
        let input: String = input.trim().to_lowercase();

        match input.as_str() {
            "jogo" => rng_game(rng.clone()),
            "numero" => rng_number_range(rng.clone()),
            "back" => return false,
            "exit" => return true,
            _ => {
                println!("Comando invalido");
                continue;
            }
        }
    }
}

fn calc_manager() {
    loop {
        println!("Digite um número:");
        println!("1: Calculadora normal");
        println!("2: Fibonacci");
        println!("3: Função 2º grau");
        println!("4: Função 3º grau");
        println!("Qualquer outro para sair");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");
        let input: &str = input.trim();

        match input {
            "1" => {
                main_calculator();
                // repetir?
                println!("Repetir? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => continue,
                    "n" => break,
                    _ => break,
                }
            }
            "2" => {
                println!("Digite o indice do fibonacci que você quer");
                println!("(Número inteiro)");
                let mut inputf = String::new();
                io::stdin()
                    .read_line(&mut inputf)
                    .expect("Failed to read number");
                let index: u32 = inputf.trim().parse::<u32>().unwrap();
                let _ = mod1::fibonacci(index);

                // repetir?
                println!("Repetir? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => continue,
                    "n" => break,
                    _ => break,
                }
            }
            "3" => {
                println!("f(x) = ax² + bx + c");
                println!("Digite o valor de a:");
                let mut inputa: String = String::new();
                io::stdin()
                    .read_line(&mut inputa)
                    .expect("Failed to read number");
                println!("Digite o valor de b:");
                let mut inputb: String = String::new();
                io::stdin()
                    .read_line(&mut inputb)
                    .expect("Failed to read number");
                println!("Digite o valor de c:");
                let mut inputc: String = String::new();
                io::stdin()
                    .read_line(&mut inputc)
                    .expect("Failed to read number");
                // parse inputs into f64:
                let a: f64 = inputa.trim().parse::<f64>().unwrap();
                let b: f64 = inputb.trim().parse::<f64>().unwrap();
                let c: f64 = inputc.trim().parse::<f64>().unwrap();

                let delta: f64 = b.powi(2) - 4.0 * a * c;

                if delta < 0.0 {
                    println!("Não há raízes reais");
                } else if delta == 0.0 {
                    let x: f64 = -b / (2.0 * a);
                    println!("Há uma raiz real: {}", x);
                } else {
                    let x1: f64 = (-b + delta.sqrt()) / (2.0 * a);
                    let x2: f64 = (-b - delta.sqrt()) / (2.0 * a);
                    println!("Há duas raízes reais: {} e {}", x1, x2);
                };
                let vertex_x: f64 = -b / (2.0 * a);
                let vertex_y: f64 = -delta / (4.0 * a);
                println!("O vértice da parábola é: ({}, {})", vertex_x, vertex_y);

                println!("Quer desenhar um gráfico? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => {
                        // println!("Digite a precisão em unidades: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let step: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Mínimo valor de x: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let x_min: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Máximo valor de x: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let x_max: f64 = input.trim().parse::<f64>().unwrap();

                        // println!("Mínimo valor de y: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let y_min: f64 = input.trim().parse::<f64>().unwrap();

                        // println!("Máximo valor de y: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let y_max: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Digite o multiplicador de resolução: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let res_mult: f64 = input.trim().parse::<f64>().unwrap();

                        mod2::f2_deg_easy(
                            a,
                            b,
                            c,
                            // step,
                            x_min as i32,
                            x_max as i32,
                            // y_min as i32,
                            // y_max as i32,
                            res_mult,
                        );
                        println!("Grafico desenhado no local do arquivo como \"plot.png\"");
                    }
                    "n" => {}
                    _ => {}
                }

                // repetir?
                println!("Repetir? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => continue,
                    "n" => break,
                    _ => break,
                }
            }
            "4" => {
                println!("f(x) = ax³ + bx² + cx + d");
                println!("Digite o valor de a:");
                let mut inputa: String = String::new();
                io::stdin()
                    .read_line(&mut inputa)
                    .expect("Failed to read number");
                println!("Digite o valor de b:");
                let mut inputb: String = String::new();
                io::stdin()
                    .read_line(&mut inputb)
                    .expect("Failed to read number");
                println!("Digite o valor de c:");
                let mut inputc: String = String::new();
                io::stdin()
                    .read_line(&mut inputc)
                    .expect("Failed to read number");
                println!("Digite o valor de d:");
                let mut inputd: String = String::new();
                io::stdin()
                    .read_line(&mut inputd)
                    .expect("Failed to read number");
                // parse inputs into f64:
                let a: f64 = inputa.trim().parse::<f64>().unwrap();
                let b: f64 = inputb.trim().parse::<f64>().unwrap();
                let c: f64 = inputc.trim().parse::<f64>().unwrap();
                let d: f64 = inputd.trim().parse::<f64>().unwrap();

                println!("Quer desenhar um gráfico? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => {
                        // println!("Digite a precisão em unidades: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let step: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Mínimo valor de x: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let x_min: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Máximo valor de x: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let x_max: f64 = input.trim().parse::<f64>().unwrap();

                        // println!("Mínimo valor de y: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let y_min: f64 = input.trim().parse::<f64>().unwrap();

                        // println!("Máximo valor de y: ");
                        // let mut input: String = String::new();
                        // io::stdin()
                        //     .read_line(&mut input)
                        //     .expect("Failed to read number");
                        // let y_max: f64 = input.trim().parse::<f64>().unwrap();

                        println!("Digite o multiplicador de resolução: ");
                        let mut input: String = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read number");
                        let res_mult: f64 = input.trim().parse::<f64>().unwrap();

                        mod2::f3_deg_easy(
                            a,
                            b,
                            c,
                            d,
                            // step,
                            x_min as i32,
                            x_max as i32,
                            // y_min as i32,
                            // y_max as i32,
                            res_mult,
                        );
                        println!("Grafico desenhado no local do arquivo como \"plot.png\"");
                    }
                    "n" => {}
                    _ => {}
                }

                // repetir?
                println!("Repetir? [S/N]");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read number");
                match input.trim().to_lowercase().as_str() {
                    "s" => continue,
                    "n" => break,
                    _ => break,
                }
            }
            _ => {
                return;
            }
        }
    }
}

fn main_calculator() {
    println!("Digite um número: ");

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
    println!("1: Adição");
    println!("2: Subtração");
    println!("3: Multiplicação");
    println!("4: Divisão");
    println!("5: Raiz quadrada");

    let mut operador: String = String::new();
    io::stdin()
        .read_line(&mut operador)
        .expect("Failed to read operator");
    let operador = operador.trim().parse::<u8>().unwrap();
    let input2: f64;
    match operador {
        0 => {
            // error, return
            println!("Please enter a valid operator");
            return;
        }
        1..=4 => {
            println!("Digite outro número");

            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read number");
            match input.trim().parse::<f64>() {
                Ok(num) => {
                    input2 = num;
                }
                Err(err) => {
                    println!("Erro: {err}");
                    return;
                }
            }
        }
        5 => {
            let res: f64 = f64::sqrt(input1);
            println!("Resposta: {res:.5}");
            return;
        } // from 5 to above, error, return
        _ => {
            println!("Please enter a valid operator");
            return;
        }
    }

    // Para editar no futuro,
    // aqui será para operações
    // com um unico número

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
    // std::thread::sleep(std::time::Duration::from_secs(1));
}

fn printer() {
    let mut input: String = String::new();

    println!("Digite o que deseja imprimir");
    io::stdin().read_line(&mut input).expect("Erro");
    println!("Você digitou: {input}");
    println!("Quer continuar? [S/N]");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "s" => printer(),
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
        println!("Digite: Calculadora, Print, Rng ou Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "calculadora" | "calc" => calc_manager(),
            "print" => printer(),
            "rng" => {
                let quitter: bool = main_rng();
                if quitter {
                    break;
                }
            }
            "exit" => break,
            _ => println!("Digite um comando válido"),
        }
    }
    // end main loop
}
