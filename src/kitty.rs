use std::collections::HashMap;

#[derive(Debug)]
struct CardInfo {
    count: u8,
    suit: String,
}

// three best possible cards in kitty
#[derive(Debug)]
struct Thani {
    cards: [[String; 3]; 3],
}

impl Thani {
    pub fn new(best_cards: [[String; 3]; 3]) -> Self {
        Thani { cards: best_cards }
    }
}

// returns collection of best kitti hand with its probable winning percentage
#[derive(Debug)]
pub struct Kitty {
    hath: [(String, String); 9],
    optimized_thani: Vec<Thani>,
}

impl Kitty {
    pub fn new(hath: Vec<String>) -> Self {
        let mut tupled_hath: Vec<(String, String)> = vec![];

        for card in hath {
            let card_split: Vec<&str> = card.split(" of ").collect();
            tupled_hath.push((String::from(card_split[0]), String::from(card_split[1])));
        }

        match tupled_hath.try_into() {
            Ok(hath) => Kitty {
                hath,
                optimized_thani: vec![],
            },
            Err(_) => {
                panic!("Failed to convert vector to array; the length of the vector does not match the required size for the array.")
            }
        }
    }

    // -- quads - no need to play; direct win
    // -- 4 pair of same number - draw/ kitti (I think)
    // -- trial -> double run -> run -> color -> pair
    pub fn get_optimized_hath(&self) {
        let hath: [(String, String); 9] = self.hath.clone();
        let mut card_count_hashmap: HashMap<String, u8> = HashMap::new();

        for card in hath {
            let counter = card_count_hashmap.entry(card.0).or_insert(0);
            *counter += 1;
        }

        // let mut hath_hashmap: HashMap<String, CardInfo> = HashMap::new();

        // for card in &self.hath {
        //     let cardsplit: Vec<&str> = card.split(" of ").collect();

        //     hath_hashmap.insert(
        //         String::from(cardsplit[0]),
        //         CardInfo {
        //             count: 1,
        //             suit: String::from(cardsplit[1]),
        //         },
        //     );
        // }

        println!("{:#?}", card_count_hashmap);
    }
}
