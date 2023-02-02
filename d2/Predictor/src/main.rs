pub mod predictor;
use predictor::Iterator;

fn main() {
    let total_score = match Iterator::new("../dataset.txt") {
        Ok(it) => {
            let mut total_score = (0, 0);
            loop {
                match it.next() {
                    Ok(round) => {
                        if let Some(round) = round {
                            total_score.0 += round.play_part_1();
                            total_score.1 += round.play_part_2();
                        } else {
                            break total_score;
                        }
                    },
                    Err(msg) => {
                        println!("error: {}", msg);
                        return;
                    },
                };
            }
        },
        Err(msg) => {
            println!("error: {}", msg);
            return;
        },
    };

    println!("predicted total score for part 1: {}, part 2: {}", total_score.0, total_score.1);
}