//! A sample stack machine that showcases how to use custom operators on custom types, and at the
//! same time, doubles as an example of how to implement a state machine.
//!
//! Put any [`Creature`][Creature] into the machine, enter one of the [`Command`s][Command], and
//! watch your creature either evolute or devolute!
//!
//! [Creature]: enum.Creature.html
//! [Command]: enum.Command.html

use crate::prelude::*;

/// Simply the first nine Pokémon.
#[derive(Clone, Debug, PartialEq)]
pub enum Creature {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Charmander,
    Charmaleon,
    Charizard,
    Squirtle,
    Wartortle,
    Blastoise,
}

/// The different commands that an evolution / devolution machine can understand.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Evolute a creature into the next stage in its evolution chart.
    Evolute,
    /// Revert a creature back to its previous stage in its evolution chart.
    Devolute,
    /// Finish operating the evolution / devolution machine.
    Close,
}

/// The main function that tells which creatures evolute and devolute into which other creatures.
pub fn pokemon_op_sys(stack: &mut Stack<Creature>, operator: &Command) {
    use Creature::*;
    let last_creature = stack.pop().unwrap();
    match operator {
        Command::Evolute => stack.push(match last_creature {
            Bulbasaur => Ivysaur,
            Ivysaur => Venusaur,
            Charmander => Charmaleon,
            Charmaleon => Charizard,
            Squirtle => Wartortle,
            Wartortle => Blastoise,
            any_other => any_other,
        }),
        Command::Devolute => stack.push(match last_creature {
            Ivysaur => Bulbasaur,
            Venusaur => Ivysaur,
            Charmaleon => Charmander,
            Charizard => Charmaleon,
            Wartortle => Squirtle,
            Blastoise => Wartortle,
            any_other => any_other,
        }),
        Command::Close => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::op_systems::pokemon::{pokemon_op_sys, Command::*, Creature::*};
    use crate::prelude::*;

    #[test]
    fn test_evolution() {
        // Let's create an evolution machine
        let mut machine = Machine::new(&pokemon_op_sys);

        // You have a Charmander
        let my_creature = Charmander;

        // Put the Charmander into the machine
        let result = machine.operate(&Item::Value(my_creature)).unwrap();

        // There should obviously be a Charmander in the machine
        assert_eq!(result, &Charmander);
        // And there should be nothing else in the machine
        assert_eq!(machine.stack_length(), 1);

        // Let's make it evolute!
        let result = machine.operate(&Item::Operator(Evolute)).unwrap();

        // Charmander should have turned into Charmaleon!
        assert_eq!(result, &Charmaleon);
        // And again there should be only 1 creature in the machine
        assert_eq!(machine.stack_length(), 1);

        // Let's evolute it again
        let result = machine.operate(&Item::Operator(Evolute)).unwrap();

        // Meet our blazing Charizard!
        assert_eq!(result, &Charizard);

        // What if we try to evolute Charizard?
        let result = machine.operate(&Item::Operator(Evolute)).unwrap();

        // Good try... but it should still be a Charizard
        assert_eq!(result, &Charizard);

        // Ok, we already got Charizard, let's just close the machine and make sure we don't leave
        // any creature behind
        machine.operate(&Item::Operator(Close));
        assert_eq!(machine.stack_length(), 0);
    }

    #[test]
    fn test_devolution() {
        // Let's create a devolution machine
        let mut machine = Machine::new(&pokemon_op_sys);

        // This time you have a Blastoise
        let my_creature = Blastoise;

        // Put the Blastoise into the machine
        let result = machine.operate(&Item::Value(my_creature)).unwrap();

        // There should obviously be a Blastoise in the machine
        assert_eq!(result, &Blastoise);
        // And there should be nothing else in the machine
        assert_eq!(machine.stack_length(), 1);

        // Let's make it devolute!
        let result = machine.operate(&Item::Operator(Devolute)).unwrap();

        // Blastoise should have turned into Wartortle!
        assert_eq!(result, &Wartortle);
        // And again there should be only 1 creature in the machine
        assert_eq!(machine.stack_length(), 1);

        // Let's devolute it again
        let result = machine.operate(&Item::Operator(Devolute)).unwrap();

        // Meet our lovely Squirtle!
        assert_eq!(result, &Squirtle);

        // What if we try to devolute Squirtle?
        let result = machine.operate(&Item::Operator(Devolute)).unwrap();

        // Good try... but it should still be a Squirtle
        assert_eq!(result, &Squirtle);

        // Ok, we already got Squirtle, let's just close the machine and make sure we don't leave
        // any creature behind
        machine.operate(&Item::Operator(Close));
        assert_eq!(machine.stack_length(), 0);
    }
}
