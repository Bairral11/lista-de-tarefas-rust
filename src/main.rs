use std::io;

fn main() {
    let mut entrada = String::new();
    let mut tarefas: Vec<String> = Vec::new(); 

    println!("Insira sua tarefa: ");
    
    io::stdin()
	    .read_line(&mut entrada)
        .expect("Entrada inválida");

    tarefas.push(entrada.trim().to_string());

    for tarefa in &tarefas {
        println!("* {tarefa}");
    }
}