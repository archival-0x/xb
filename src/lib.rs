/// ## XB Turing Machine
/// Our machine is comprised of two cells:
///
///     | B | B |
///
/// The purpose of this machine is to make it such that it is capable of changing the first cell to an X, and then reverting back to a  B.
///
///   Pseudocode Instruction Set:
///     R       = move cell to the right
///     L       = move cell to the left
///     ->      = alter content of cell
///     s1..s4  = state symbols
///
/// Our example program:
///     B, s1 -> X, R, s2
///     B, s2 -> B, L, s3
///     X, s3 -> B, R, s4
///     B, s4 -> B, L, s1
///
///
/// Note the similarities of the design and implementation of a finite state machine.
use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash, Debug)]
struct State<'a> {
    current_char: char,
    current_state: &'a str,
}

#[derive(Debug)]
struct Instruction<'a> {
    symbol: char,
    direction: char,
    next_state: &'a str,
}

/// define a type alias to Hashmap<State, Instruction>
type XbMachine<'a> = HashMap<State<'a>, Instruction<'a>>;

/// initialize a trait that extends methods for our type aliased XbMachine
trait XbExt {
    fn new(
        vec_state: Vec<(char, &'static str)>,
        vec_instruction: Vec<(char, char, &'static str)>,
    ) -> Self;
    fn simulate(&mut self);
}

impl<'a> XbExt for XbMachine<'a> {
    /// When creating a new XB Machine:
    ///    - specify a vector of 2-tuples with all the states
    ///    - specify a vectory of 3-tuples with instructions for the states
    ///
    /// ... from that, create a new XbMachine HashMap type, where each state is mapped
    /// onto each instruction.
    fn new(
        vec_state: Vec<(char, &'a str)>,
        vec_instruction: Vec<(char, char, &'a str)>,
    ) -> XbMachine<'a> {
        // initialize hashmap (eventually the XbMachine type)
        let mut isa = HashMap::new();

        for i in 0..vec_state.len() {
            // Create the new State from the first tuple provided
            let state = State {
                current_char: vec_state[i].0,
                current_state: vec_state[i].1,
            };

            // Create the new Instruction from the second tuple provided
            let instruction = Instruction {
                symbol: vec_instruction[i].0,
                direction: vec_instruction[i].1,
                next_state: vec_instruction[i].2,
            };

            // Insert both into the HashMap
            isa.insert(state, instruction);
        }
        isa
    }

    /// given a tape and instruction set, execute the given instructions and exit.
    fn simulate(&mut self) {
        // declare tape with initial state
        let mut tape: Vec<char> = vec!['B', 'B'];

        // initialize head "register"
        let mut head: isize = 0;

        // Set the default state to start in.
        let mut index_state: &str = "s1";

        // Iterate over the HashMap by unpacking the key (State) and value (Instruction)
        for (state, instruction) in self.iter() {
            // Set the default state as the initial symbol and the state
            let index_key = (tape[head as usize], index_state);

            // Check if a tuple comprising of the elements of a key is equal to index_key.
            if (state.current_char, state.current_state) == index_key {
                // Print the current state of the tape.
                println!("{:?}", tape);

                // Output the current instruction being executed.
                println!(
                    "{:?}, {:?} -> {:?}, {:?}, {:?}",
                    state.current_char,
                    state.current_state,
                    instruction.symbol,
                    instruction.direction,
                    instruction.next_state
                );

                // Now, set the cell in the tape to the specified symbol
                tape[head as usize] = instruction.symbol;

                // Move to the cell as specified by the direction
                match instruction.direction {
                    'R' => head += 1,
                    _ => head += -1,
                }

                // This means that the head has fallen through the cracks of
                // unsigned-signed space and time.
                if head < 0 {
                    head = 1;
                } else if head > 2 {
                    head = 0;
                }

                // Assign state to the new state as specified by instruction
                index_state = instruction.next_state;
            }
        }
    }
}

mod tests {

    #[test]
    fn it_works() {
        // initialize our states
        let xb_states = vec![('B', "s1"), ('B', "s2"), ('X', "s3"), ('B', "s4")];

        // create our instructions
        let xb_instructions = vec![
            ('X', 'R', "s2"),
            ('B', 'L', "s3"),
            ('B', 'R', "s4"),
            ('B', 'L', "s1"),
        ];

        // create a new machine, and simulate
        let mut xb_isa = XbMachine::new(xb_states, xb_instructions);
        xb_isa.simulate();
    }
}
