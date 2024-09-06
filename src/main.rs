use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct DFA {
    states: HashSet<Node>,
    alphabet: HashSet<char>,
    start_state: Node,
    // transitions: HashSet<Transition>,
    accept_states: HashSet<Node>,
}

struct Node {
    state: String,
    is_accept: bool,
    transitions: HashMap<char, Rc<RefCell<Node>>>,
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

fn main() {
    let q0 = Node::new("q0", false);
    let q1 = Node::new("q1", true);

    Node::add_transition(&q0, '0', q0.clone());
    Node::add_transition(&q0, '1', q1.clone());

    Node::add_transition(&q1, '0', q0.clone());
    Node::add_transition(&q1, '1', q1.clone());

    let mut current_state = q0.clone(); // Use clone to get the Rc reference
    let input = "1101";

    for c in input.chars() {
        let next_state = current_state.borrow().next_state(c);

        match next_state {
            Some(next) => {
                current_state = next;
            }
            None => {
                println!("Invalid input");
                return;
            }
        }
    }

    // Check if the final state is an accept state
    if current_state.borrow().is_accept {
        println!("Accepted");
    } else {
        println!("Rejected");
    }
}
