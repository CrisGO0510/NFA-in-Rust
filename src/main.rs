use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::process::Command;
use std::rc::Rc;

struct DFA {
    states: Vec<Rc<RefCell<Node>>>,
    alphabet: HashSet<char>,
    start_state: Rc<RefCell<Node>>,
}

struct Node {
    state: String,
    is_accept: bool,
    transitions: HashMap<char, Rc<RefCell<Node>>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Iniciar el formato con el nombre de la estructura
        write!(f, "Node {{\n")?;
        // Escribir el campo state
        write!(f, "    state: \"{}\",\n", self.state)?;
        // Escribir el campo is_accept
        write!(f, "    is_accept: {},\n", self.is_accept)?;
        // Escribir las transiciones con un formato detallado
        write!(f, "    transitions: {{\n")?;
        for (key, value) in &self.transitions {
            write!(f, "        '{}': \"{}\",\n", key, value.borrow().state)?;
        }
        // Cerrar la llave de transitions y Node
        write!(f, "    }},\n")?;
        write!(f, "}}") // Finalizar el formato
    }
}

impl Node {
    fn new(state: &str, is_accept: bool) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            state: state.to_string(),
            is_accept,
            transitions: HashMap::new(),
        }))
    }

    fn add_transition(node: &Rc<RefCell<Node>>, symbol: char, to: Rc<RefCell<Node>>) {
        node.borrow_mut().transitions.insert(symbol, to);
    }

    fn next_state(&self, symbol: char) -> Option<Rc<RefCell<Node>>> {
        self.transitions.get(&symbol).cloned()
    }
}

impl DFA {
    fn new() -> DFA {
        let alphabet = create_alphabet();
        let states = create_states();
        create_transitions(&states, &alphabet);
        let start_state = define_start_states(&states);

        DFA {
            states,
            alphabet,
            start_state,
        }
    }

    fn run(&self, input: &str) -> bool {
        let mut current_state = self.start_state.clone();

        for c in input.chars() {
            let next_state = current_state.borrow().next_state(c);

            match next_state {
                Some(next) => {
                    current_state = next;
                }
                None => {
                    println!("No hay transición para el simbolo {}", c);
                    return false;
                }
            }
        }

        // Check if the final state is an accept state
        if current_state.borrow().is_accept {
            return true;
        }

        return false;
    }

    fn print_states(&self) {
        print!("{{");
        let mut first = true;
        for state in &self.states {
            if !first {
                print!(", ");
            }
            print!("{}", state.borrow().state);
            first = false;
        }
        print!("}}");
    }

    fn print_alphabet(&self) {
        print!("{:?}", self.alphabet);
    }

    fn print_start_state(&self) {
        print!("{}", self.start_state.borrow().state);
    }

    fn print_accept_states(&self) {
        print!("{{");
        let mut first = true;
        for state in &self.states {
            if state.borrow().is_accept {
                if !first {
                    print!(", ");
                }
                print!("{}", state.borrow().state);
                first = false;
            }
        }
        print!("}}");
    }

    fn tupla(&self) {
        print!("A = <");

        // Imprimir Q
        print!("Q = ");
        Self::print_states(&self);
        print!(", ");

        // Imprimir Σ
        print!("Σ = ");
        Self::print_alphabet(&self);
        print!(", ");

        // Imprimir el estado inicial
        Self::print_start_state(&self);

        // Imprimir δ (transiciones) - Asumiendo que tienes una función para esto o lo omites por ahora
        print!(", δ, ");

        // Imprimir F
        print!("F = ");
        Self::print_accept_states(&self);
        println!(">");
    }
}

fn main() {
    menu();

    println!("Gracias por usar el programa.");
}

fn create_alphabet() -> HashSet<char> {
    let mut alphabet = HashSet::new();
    let size: usize;
    let mut input;

    loop {
        println!("Ingrese la cardinalidad del alfabeto: ");
        input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        size = match input.trim().parse() {
            Ok(size) => size,
            Err(_) => {
                println!("Error al leer la entrada.");
                continue;
            }
        };
        break;
    }

    let mut i = 0;

    while i < size {
        println!("Ingrese el simbolo {}: ", i + 1);
        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue; // No incrementa i si hay error en la lectura
        }

        let symbol = match input.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Entrada vacía, por favor ingrese un símbolo.");
                continue; // No incrementa i si no se ingresa un símbolo
            }
        };

        if alphabet.insert(symbol) {
            i += 1; // Solo incrementa i si el símbolo es válido y no está duplicado
        } else {
            println!("El simbolo ya existe en el alfabeto.");
            // No se incrementa i si el símbolo ya existe en el alfabeto
        }
    }
    alphabet
}

fn create_states() -> Vec<Rc<RefCell<Node>>> {
    let mut states = Vec::new();
    let mut states_name: HashSet<String> = HashSet::new();
    let mut input;

    loop {
        println!("Ingrese la cardinalidad del conjunto de los estados: ");
        input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        let size = match input.trim().parse() {
            Ok(size) => size,
            Err(_) => {
                println!("Error al leer la entrada.");
                continue;
            }
        };

        let mut i: usize = 0;

        while i < size {
            // for i in 0..size {
            println!("Ingrese el nombre del estado {}: ", i);
            input = String::new();

            if std::io::stdin().read_line(&mut input).is_err() {
                println!("Error al leer la entrada.");
                continue;
            }

            let state = input.trim().to_string();

            if !states_name.insert(state.clone()) {
                println!("El estado \"{}\" ya ha sido definido.", state);
                continue;
            }

            let is_accept;

            loop {
                println!("Es estado de aceptacion? (s/n): ");
                input = String::new();

                if std::io::stdin().read_line(&mut input).is_err() {
                    println!("Error al leer la entrada.");
                    continue;
                }

                match input.trim() {
                    "s" => {
                        is_accept = true;
                        break;
                    }
                    "n" => {
                        is_accept = false;
                        break;
                    }
                    _ => {
                        println!("Opcion invalida.");
                    }
                }
            }

            states.push(Node::new(&state, is_accept));
            i += 1;
        }

        break;
    }

    states
}

fn create_transitions(states: &Vec<Rc<RefCell<Node>>>, alphabet: &HashSet<char>) {
    for state in states {
        for symbol in alphabet.clone().into_iter() {
            let mut input;

            loop {
                println!(
                    "Ingrese el estado al que se transiciona \"{}\" con el simbolo {}: ",
                    state.borrow().state,
                    symbol
                );
                input = String::new();

                if std::io::stdin().read_line(&mut input).is_err() {
                    println!("Error al leer la entrada.");
                    continue;
                }

                let next_state = states.iter().find(|&x| x.borrow().state == input.trim());

                match next_state {
                    Some(next) => {
                        Node::add_transition(&state, symbol, next.clone());
                        break;
                    }
                    None => {
                        println!("El estado no existe.");
                    }
                }
            }
        }
    }
}

fn define_start_states(states: &Vec<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    let mut input;

    if states.is_empty() {
        return Node::new("Empty", false);
    }

    loop {
        println!("Ingrese el estado inicial: ");
        input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        let start_state = states.iter().find(|&x| x.borrow().state == input.trim());

        match start_state {
            Some(start) => {
                return start.clone();
            }
            None => {
                println!("El estado no existe.");
            }
        }
    }
}

fn menu() {
    clear_console();
    println!("Cree un autómata finito determinista.\n");
    let mut dfa = DFA::new();

    loop {
        wait_for_keypress();
        clear_console();
        println!("Automata Finito Determinista");
        println!("=============================");
        println!("1. Crear o reemplazar un nuevo autómata.");
        println!("2. Validar una palabra.");
        println!("3. Imprimir el conjunto de estados.");
        println!("4. Imprimir el alfabeto.");
        println!("5. Imprimir el estado inicial.");
        println!("6. Imprimir los conjuntos de aceptación.");
        println!("7. Imprimir las 5-tupla.");
        println!("8. Salir del programa.\n");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                dfa = DFA::new();
                println!("Nuevo autómata creado.");
            }
            "2" => {
                println!("Ingrese la palabra a validar:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if dfa.run(input) {
                    println!("La palabra es aceptada por el autómata.");
                } else {
                    println!("La palabra es rechazada por el autómata.");
                }
            }
            "3" => {
                println!("Conjunto de estados:");
                dfa.print_states();
                println!();
            }
            "4" => {
                println!("Alfabeto:");
                dfa.print_alphabet();
                println!();
            }
            "5" => {
                println!("Estado inicial:");
                dfa.print_start_state();
                println!();
            }
            "6" => {
                println!("Conjuntos de aceptación:");
                dfa.print_accept_states();
                println!();
            }
            "7" => {
                println!("Conjuntos de aceptación:");
                dfa.tupla();
            }
            "8" => break,
            _ => println!("Opción no válida, intente de nuevo."),
        }
    }
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error al limpiar la consola");
    } else {
        Command::new("clear")
            .status()
            .expect("Error al limpiar la consola");
    }
}

fn wait_for_keypress() {
    let mut input = String::new();
    println!("Presione enter para continuar...");
    std::io::stdin().read_line(&mut input).ok();
}
