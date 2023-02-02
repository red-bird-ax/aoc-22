pub mod predictor;
use predictor::Iterator;

fn main() {

    let total_score = match Iterator::new("../dataset.txt") {
        Ok(it) => {
            let mut total_score = 0;
            loop {
                match it.next() {
                    Ok(round) => {
                        if let Some(round) = round {
                            total_score += round.play();
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

    println!("predicted total score: {}", total_score);
}