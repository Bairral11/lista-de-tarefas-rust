use std::io::{self, Write};

fn main() {
    let _entrada = String::new();
    let mut tarefas: Vec<String> = Vec::new(); 
    
    loop {
        let mut op = String::new();

        print!("\n> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut op)
            .expect("entrada inválida\n");

        match op.trim() {
            "help" => help(),
            "mk" => nova_tarefa(&mut tarefas),
            "rm" => remover_tarefa(&mut tarefas),
            "ls" => listar_tarefas(&tarefas),
            "n" => count(&tarefas),
            "exit" => {
                println!("Encerrando...");
                break;
            }
            _ => println!("Operação inválida \n"),
        }
    }
}

fn help() {
    println!("
* help - Lista comandos.
* mk - Criar nova tarefa.
* rm - Remover tarefa.
* ls - Listar tarefas.
* n - Retornar numero de tarefas.
* exit - Fechar programa"
    );
}

fn nova_tarefa(tarefas: &mut Vec<String>) {
    let mut entrada = String::new();

    print!("\nInsira sua tarefa: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut entrada).expect("Entrada inválida");
    tarefas.push(entrada.trim().to_string());
}

fn listar_tarefas(tarefas: &[String]) {
    for tarefa in tarefas {
        println!("* {tarefa}");
    }
}

fn count(tarefas: &[String]) {
    println!("\nTotal de tarefas: {}", tarefas.len());
}

fn remover_tarefa(tarefas: &mut Vec<String>) {
    let mut entrada: String = String::new();

    print!("\nQual tarefa deseja remover: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut entrada).expect("Entrada inválida");

    for (indice, tarefa) in tarefas.iter().enumerate() {
        if entrada.trim() == tarefa {
            tarefas.remove(indice);
            println!("\nTarefa removida.");
            return;
        }
    }

    println!("\nTarefa não encontrada.");
}