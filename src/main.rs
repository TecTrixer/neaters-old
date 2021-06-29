mod tictactoe;
mod genotype;

use crate::genotype::*;

fn main() {
    /*
    loop of
    let res = game.move(player1.decide(board));
    if res == invalid {
        player1.fitness == number_of_moves
        player2.fitness == number_of_moves * 4
    }
    if res == winner {
        player1.fitness = 150 - number_of_moves * 2
        player2.fitness = number_of_moves * 3
    }  

    same with player 2 ...

    start loop again
    */
    let nn = Genotype::new(3, 2);
    println!("{:?}", nn.save(None));
}
