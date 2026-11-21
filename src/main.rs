use std::io;

fn main() {
    let mut _entrada = String::new();
    let mut tarefas: Vec<String> = Vec::new(); 
    
    loop {
        let mut op = String::new();
        
        println!("\nOperações: \n* mk - criar nova tarefa\n* ls - listar tarefas\n* exit - fecha o programa\n");
        io::stdin()
            .read_line(&mut op)
            .expect("entrada inválida\n");

        match op.trim() {
            "mk" => nova_tarefa(&mut tarefas),
            "ls" => listar_tarefas(&tarefas),
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
    io::stdin()
        .read_line(&mut entrada)
        .expect("Entrada inválida\n");

    tarefas.push(entrada.trim().to_string());
}

fn listar_tarefas(tarefas: &[String]) {
    for tarefa in tarefas {
        println!("* {tarefa}");
    }
}