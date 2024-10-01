use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::rc::Rc;

struct NFA {
    states: Vec<Rc<RefCell<Node>>>,
    alphabet: HashSet<char>,
    start_state: Rc<RefCell<Node>>,
}

struct Node {
    state: String,
    is_accept: bool,
    transitions: HashMap<char, Vec<Rc<RefCell<Node>>>>,
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
        node.borrow_mut()
            .transitions
            .entry(symbol)
            .or_insert(Vec::new())
            .push(to);
    }

    fn next_states(&self, symbol: char) -> Vec<Rc<RefCell<Node>>> {
        self.transitions
            .get(&symbol)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}

impl NFA {
    fn new() -> NFA {
        let alphabet = create_alphabet();
        let states = create_states();
        create_transitions(&states, &alphabet);
        let start_state = define_start_states(&states);

        NFA {
            states,
            alphabet,
            start_state,
        }
    }

    /**
     Valida si la palabra ingresada es aceptada por el autómata.
        # Arguments
        * `input` - La palabra a analizar.
        # Returns
        Retorna un `bool` que puede determinar si la palabra es aceptada o no por el autómata.
    */
    fn run(&self, input: &str) -> bool {
        let mut paths: Vec<Vec<Rc<RefCell<Node>>>> = vec![vec![self.start_state.clone()]];
    
        for c in input.chars() {
            let mut new_paths = Vec::new();
    
            for path in paths {
                if let Some(last_state) = path.last() {
                    let possible_states = last_state.borrow().next_states(c);
    
                    if possible_states.is_empty() {
                        println!("No hay transición para el símbolo {}", c);
                        continue;
                    }
    
                    for next_state in possible_states {
                        let mut new_path = path.clone();
                        new_path.push(next_state.clone());
                        new_paths.push(new_path);
                    }
                }
            }
    
            if new_paths.is_empty() {
                println!("No hay transiciones posibles para el símbolo {}", c);
                return false;
            }
    
            paths = new_paths;
        }
    
        let mut is_accepted = false;
        for path in &paths {
            if let Some(last_state) = path.last() {
                if last_state.borrow().is_accept {
                    is_accepted = true;
                    println!("Ruta aceptada: {:?}", path.iter().map(|s| s.borrow().state.clone()).collect::<Vec<_>>());
                }
            }
        }
    
        if !is_accepted {
            println!("La palabra no es aceptada por ninguna ruta.");
        }
    
        is_accepted
    }
    
    // Imprime el conjunto de estados
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

    // Imprime el alfabeto
    fn print_alphabet(&self) {
        print!("{:?}", self.alphabet);
    }

    // Imprime el estado inicial
    fn print_start_state(&self) {
        print!("{}", self.start_state.borrow().state);
    }

    // Imprime el conjunto de estados de aceptación
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

    // Imprime las transiciones del autómata NFA
    fn print_transitions(&self) {
        for state in &self.states {
            let state_borrow = state.borrow();
            for (symbol, next_states) in &state_borrow.transitions {
                let next_states_str: Vec<String> = next_states
                    .iter()
                    .map(|next_state| next_state.borrow().state.clone())
                    .collect();

                println!(
                    "δ({}, {}) = [{}]",
                    state_borrow.state,
                    symbol,
                    next_states_str.join(", ")
                );
            }
        }
    }

    // Imprime la 5-tupla (Definición formal de un NFA)
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

/**
 Función que crea el alfabeto del autómata.
    # Returns
    Retorna un `HashSet<char>` el cual representa mi conjunto de símbolos .
*/
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
        println!("Ingrese el símbolo {}: ", i + 1);
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
            println!("El símbolo ya existe en el alfabeto.");
            // No se incrementa i si el símbolo ya existe en el alfabeto
        }
    }
    alphabet
}

/**
 Función que crea mi conjunto de estados del autómata .
    # Returns
    Retorna un `Vec<Rc<RefCell<Node>>>` el cual representa mi conjunto de estados.
*/
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

/**
 Crea las transiciones entre los nodos (estados) del autómata.
    # Arguments
    * `states` - Referencia al vector de nodos.
    * `alphabet` - Referencia al alfabeto.
*/
fn create_transitions(states: &Vec<Rc<RefCell<Node>>>, alphabet: &HashSet<char>) {
    loop {
        // Solicitar al usuario el estado-símbolo-estados_destino
        println!("Ingrese la transición en el formato \"(estado_actual, símbolo)->{{estados_destino}}\" (o escriba \"exit\" para salir):");
        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        let input = input.trim();

        // Verificar si el usuario quiere salir
        if input.to_lowercase() == "exit" {
            break;
        }

        // Verificar el formato de la entrada
        let parts: Vec<&str> = input.split("->").collect();
        if parts.len() != 2 {
            println!("Formato incorrecto. Debe ser \"(estado_actual, símbolo)->{{estados_destino}}\".");
            continue;
        }

        let transition_part = parts[0].trim();
        let next_states_input = parts[1].trim();

        // Verificar que el formato de la parte de transición sea correcto (debe ser "(estado_actual, símbolo)")
        if !transition_part.starts_with('(') || !transition_part.ends_with(')') {
            println!("Formato incorrecto en la parte de transición. Debe ser \"(estado_actual, símbolo)\".");
            continue;
        }

        // Remover los paréntesis y dividir por la coma
        let transition_inner = &transition_part[1..transition_part.len() - 1];
        let transition_parts: Vec<&str> = transition_inner.split(',').collect();

        if transition_parts.len() != 2 {
            println!("Formato incorrecto. Debe haber un estado y un símbolo separados por coma.");
            continue;
        }

        let state_input = transition_parts[0].trim();
        let symbol_input = transition_parts[1].trim();

        // Verificar que el símbolo tenga un solo carácter
        if symbol_input.len() != 1 {
            println!("El símbolo debe ser un solo carácter.");
            continue;
        }

        let symbol = symbol_input.chars().next().unwrap();

        // Verificar que el símbolo pertenezca al alfabeto
        if !alphabet.contains(&symbol) {
            println!("El símbolo '{}' no pertenece al alfabeto.", symbol);
            continue;
        }

        // Verificar que los estados destino estén entre llaves
        if !next_states_input.starts_with('{') || !next_states_input.ends_with('}') {
            println!("Formato incorrecto en los estados destino. Deben estar dentro de llaves \"{{estado1, estado2}}\".");
            continue;
        }

        // Remover las llaves y dividir los estados destino por comas
        let next_states_inner = &next_states_input[1..next_states_input.len() - 1];
        let next_states: Vec<&str> = next_states_inner.split(',').map(|s| s.trim()).collect();

        // Buscar el estado actual
        let current_state = states.iter().find(|&x| x.borrow().state == state_input);

        if let Some(current) = current_state {
            // Buscar y agregar las transiciones para cada estado destino
            for next_state_name in next_states {
                let next_state = states.iter().find(|&x| x.borrow().state == next_state_name);

                match next_state {
                    Some(next) => {
                        // Agregar la transición a cada estado destino
                        Node::add_transition(current, symbol, next.clone());
                        println!("Transición agregada: δ({}, {}) = {}", state_input, symbol, next_state_name);
                    }
                    None => {
                        println!("El estado destino \"{}\" no existe.", next_state_name);
                    }
                }
            }
        } else {
            println!("El estado \"{}\" no existe.", state_input);
        }
    }
}


/**
 Define mi estado inicial del autómata.
    # Arguments
    * `states` - Referencia al vector de nodos.
    # Returns
    Retorna un `Rc<RefCell<Node>>` que es el puntero al estado inicial.
*/
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

// Menú principal del programa.
fn menu() {
    clear_console();
    println!("Cree un autómata finito No determinista.\n");
    let mut nfa = NFA::new();

    loop {
        wait_for_keypress();
        clear_console();
        println!("Autómata Finito No Determinista");
        println!("=============================");
        println!("1. Crear o reemplazar un nuevo autómata.");
        println!("2. Validar una palabra.");
        println!("3. Imprimir el conjunto de estados.");
        println!("4. Imprimir el alfabeto.");
        println!("5. Imprimir el estado inicial.");
        println!("6. Imprimir los conjuntos de aceptación.");
        println!("7. Imprimir las 5-tupla.");
        println!("8. Imprimir las transiciones.");
        println!("9. Salir del programa.\n");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                nfa = NFA::new();
                println!("Nuevo autómata creado.");
            }
            "2" => {
                println!("Ingrese la palabra a validar:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if nfa.run(input) {
                    println!("La palabra es aceptada por el autómata.");
                } else {
                    println!("La palabra es rechazada por el autómata.");
                }
            }
            "3" => {
                println!("Conjunto de estados:");
                nfa.print_states();
                println!();
            }
            "4" => {
                println!("Alfabeto:");
                nfa.print_alphabet();
                println!();
            }
            "5" => {
                println!("Estado inicial:");
                nfa.print_start_state();
                println!();
            }
            "6" => {
                println!("Conjuntos de aceptación:");
                nfa.print_accept_states();
                println!();
            }
            "7" => {
                println!("Conjuntos de aceptación:");
                nfa.tupla();
            }
            "8" => {
                println!("Transiciones:");
                nfa.print_transitions();
            }
            "9" => break,
            _ => println!("Opción no válida, intente de nuevo."),
        }
    }
}

// Función para limpiar la consola
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

// Función para esperar a que el usuario presione una tecla
fn wait_for_keypress() {
    let mut input = String::new();
    println!("Presione enter para continuar...");
    std::io::stdin().read_line(&mut input).ok();
}
