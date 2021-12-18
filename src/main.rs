
#[derive(Copy, Clone, Debug)]
struct Hand {
    value: i32,
    aces: bool,
}
impl Hand {
    fn score(&self) -> i32 {
        if self.value <= 11 && self.aces {
            self.value + 10
        } else {
            self.value
        }
    }
}
impl std::ops::Add for Hand {
    type Output = Hand;
    fn add(mut self, rhs: Hand) -> Hand {
        self.value += rhs.value;
        self.aces = self.aces || rhs.aces;
        self
    }
}

fn solve<'a>(values_raw: &'a str) -> i32 {
    let mut values_vec: Vec<_> = values_raw.chars().collect();
    let values: Vec<_> =  values_raw.chars().map(|c| {
        match c {
            'A' => Hand { value: 1, aces: true },
            '2' => Hand { value: 2, aces: false },
            '3' => Hand { value: 3, aces: false },
            '4' => Hand { value: 4, aces: false },
            '5' => Hand { value: 5, aces: false },
            '6' => Hand { value: 6, aces: false },
            '7' => Hand { value: 7, aces: false },
            '8' => Hand { value: 8, aces: false },
            '9' => Hand { value: 9, aces: false },
            'T' => Hand { value: 10, aces: false },
            'J' => Hand { value: 10, aces: false },
            'Q' => Hand { value: 10, aces: false },
            'K' => Hand { value: 10, aces: false },
            c => unreachable!(c),
        }
    }).collect();

    let mut max_scores = vec![-999; values.len()];
    let mut hits = vec![(-1, -1); values.len()];
    let mut results = vec![-2; values.len()];

    for i in (0..values.len()).rev() {
        if i + 4 > values.len() {
            max_scores[i] = 0;
            continue;
        }

        let player_initial = values[i] + values[i+1];
        let dealer_initial = values[i+2] + values[i+3];

        let mut player_can_end = false;
        let mut dealer_can_end = false;

        let mut player_hand = player_initial;
        'outer: for player_hits in 0.. {
            if i + 3 + player_hits >= values.len() {
                player_can_end = true;
                break;
            }

            if player_hits > 0 {
                player_hand = player_hand + values[i + 4 + (player_hits - 1)];
            }
            
            let mut dealer_hits = 0;
            let mut dealer_hand = dealer_initial;
            if player_hand.score() <= 21 {
                while dealer_hand.score() < 17 {
                    dealer_hits += 1;
                    if i + 3 + player_hits + dealer_hits == values.len() {
                        dealer_can_end = true;
                        break 'outer;
                    }
                    dealer_hand = dealer_hand + values[i + 4 + player_hits + (dealer_hits - 1)];
                }
            }

            let result = if player_hand.score() > 21 {
                -1
            } else if dealer_hand.score() > 21 || player_hand.score() > dealer_hand.score() {
                1
            } else if player_hand.score() == dealer_hand.score() {
                0
            } else {
                -1
            };

            let next_index = i + 4 + player_hits + dealer_hits;
            let score = if next_index == values.len() {
                max_scores[i].max(result)
            } else {
                max_scores[i].max(result + max_scores[next_index])
            };

            if score > max_scores[i] {
                max_scores[i] = score;
                hits[i] = (player_hits as i32, dealer_hits as i32);
                results[i] = result;
            }

            if player_hand.score() > 21 {
                break;
            }
        }

        if player_can_end && max_scores[i] < 0 {
            max_scores[i] = 0;
            hits[i] = (-2, -2);
            results[i] = 0;
        }
        if dealer_can_end && max_scores[i] < 0 {
            max_scores[i] = 0;
            hits[i] = (-3, -3);
            results[i] = 0;
        }
    }

    println!("{}", values_raw);
    let mut i = 0;
    while i < hits.len() {
        let (player_hits, dealer_hits) = hits[i];
        if player_hits >= 0 && dealer_hits >= 0 {
            print!("{}: player={}{}", i, values_vec[i], values_vec[i+1]);
            for j in 0..player_hits {
                print!("{}", values_vec[i+4+j as usize]);
            }
            print!(", dealer={}{}", values_vec[i+2], values_vec[i+3]);
            for j in 0..dealer_hits {
                print!("{}", values_vec[i+4+(player_hits+j) as usize]);
            }
            if results[i] == 1 {
                println!(" (player wins)");
            } else if results[i] == 0 {
                println!(" (tie)");
            } else {
                println!(" (dealer wins)");
            }
            i += (4 + player_hits + dealer_hits) as usize;
        } else if player_hits == -2 {
            println!("{}: DECK EXHAUSTED (tie)", i);
            break;
        } else if player_hits == -3 {
            println!("{}: DECK EXHAUSTED (tie)", i);
            break;
        } else {
            break;
        }

    }

    max_scores[0]
}

fn main() {
    let deals = [
        "A9K87492QKJ398456KQ8A395J42TKA85639KKAJ55784A92",
        "A85639K8KKAJ5KAJ5A9A85639A9763TK6K2Q3487492QKJ2AA9TQ77886",
        "TKA85639KKAJ5A9K87492QKJ39A7AA8Q43K86563JJT84393KKKJ9",
        "A8Q43K89K87495639KKAJAAAAAKQ492J56KJA397745835KK63KQ",
        "2QKJ39A7A492QK593JK4342AA9TQ78492QK56893JK4342AA9TQ77886563JJTA423588",
        "8456KQ8A395J42TKA85639KKAJ5",
        "AA8Q43K8656639KKAJ2QKJ39A73JJT84393",
        "A9K87492QKJ399KKAJAAAAAKQ492J56KJA39774A9K87492QKJ39K593JK4342AA9TQ78A9K87492QKJ3992QK593JK4342AAA9K87492QKJ39QKJ39A7A492QK59384",
        "A9K87492QKJ395A9K874TKA8593K8656KJ395A9K874T6437KKK624QJ44",
        "A7AA8Q43K868492QK593JK4342AA9TQ77886563T5AAA9435JJ23J6839J",
    ];
    for (index, deal) in deals.into_iter().enumerate() {
        println!();
        println!("======== DECK #{} ========", index + 1);
        println!("score = {}", solve(deal));
    }
}
