use std::io::stdin;

fn main() {
    let _entrada = String::new();
    let mut tarefas: Vec<String> = Vec::new(); 
    
    loop {
        let mut op = String::new();
        
        println!("\nOperações: \n* mk - criar nova tarefa\n* ls - listar tarefas\n* rm - Remover uma tarefa\n* exit - fecha o programa\n");
        stdin()
            .read_line(&mut op)
            .expect("entrada inválida\n");

        match op.trim() {
            "mk" => nova_tarefa(&mut tarefas),
            "ls" => listar_tarefas(&tarefas),
            "rm" => remover_tarefa(&mut tarefas),
            "exit" => {
                println!("Encerrando...");
                break;
            }
            _ => println!("Operação inválida \n"),
        }
    }
}

fn nova_tarefa(tarefas: &mut Vec<String>) {
    let mut entrada = String::new();

    println!("Insira sua tarefa: ");
    stdin().read_line(&mut entrada).expect("Entrada inválida");

    tarefas.push(entrada.trim().to_string());
}

fn listar_tarefas(tarefas: &[String]) {
    for tarefa in tarefas {
        println!("* {tarefa}");
    }
}

fn remover_tarefa(tarefas: &mut Vec<String>) {
    let mut entrada: String = String::new();

    println!("Qual tarefa deseja remover: ");
    stdin().read_line(&mut entrada).expect("Entrada inválida");

    for (indice, tarefa) in tarefas.iter().enumerate() {
        if entrada.trim() == tarefa {
            tarefas.remove(indice);
            println!("Tarefa removida.");
            return;
        }
    }

    println!("Tarefa não encontrada.");
}