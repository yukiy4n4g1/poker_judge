use itertools::Itertools;

enum PokerHands {
    StraightFlush,
    FourOfKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfKind,
    TwoPair,
    Pair,
    HighCard,
}

impl PokerHands {
    fn judge(cards: &Vec<Card>) -> PokerHands {
        let ranks: Vec<_> = cards.into_iter().map(|c| c.rank).sorted().collect();
        let suits: Vec<_> = cards.into_iter().map(|c| c.suit).collect();
        let is_straight = ranks == (ranks[0]..ranks[0] + 5).collect::<Vec<_>>()
            || ranks == vec![1, 10, 11, 12, 13];
        let is_flush = cards.into_iter().all(|c| c.suit == cards[0].suit);
        let mut count = vec![0; 13];
        for i in suits {
            count[i] += 1;
        }
        let hand: Vec<_> = count
            .into_iter()
            .enumerate()
            .filter(|x| x.1 > 0)
            .sorted_by_key(|x| -x.1)
            .map(|x| x.1)
            .collect();

        if is_flush && is_straight {
            PokerHands::StraightFlush
        } else if hand == &[4, 1] {
            PokerHands::FourOfKind
        } else if hand == &[3, 2] {
            PokerHands::FullHouse
        } else if is_flush {
            PokerHands::Flush
        } else if is_straight {
            PokerHands::Straight
        } else if hand == &[3, 1, 1] {
            PokerHands::ThreeOfKind
        } else if hand == &[2, 2, 1] {
            PokerHands::TwoPair
        } else if hand == &[2, 1, 1, 1] {
            PokerHands::Pair
        } else {
            PokerHands::HighCard
        }
    }

    fn to_string_ja(&self) -> String {
        let s = match self {
            PokerHands::StraightFlush => "ストレートフラッシュ",
            PokerHands::FourOfKind => "フォーカード",
            PokerHands::FullHouse => "フルハウス",
            PokerHands::Flush => "フラッシュ",
            PokerHands::Straight => "ストレート",
            PokerHands::ThreeOfKind => "スリーカード",
            PokerHands::TwoPair => "ツーペア",
            PokerHands::Pair => "ワンペア",
            PokerHands::HighCard => "ハイカード",
        };
        s.to_string()
    }
}

struct Card {
    suit: usize,
    rank: usize,
}

impl Card {
    fn new(s: usize, r: usize) -> Card {
        Card { suit: s, rank: r }
    }

    fn to_string(&self) -> String {
        format!(
            "{}{}",
            match self.suit % 4 {
                0 => 'S',
                1 => 'C',
                2 => 'D',
                3 => 'H',
                _ => ' ',
            },
            match self.rank {
                1 => "A".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => self.rank.to_string(),
            }
        )
    }
}

fn main() {
    let mut cards: Vec<Card> = vec![];
    for _ in 0..5 {
        let (s, r) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();

            let mut ws = s.trim_end().split_whitespace();
            let s: usize = ws.next().unwrap().parse().unwrap();
            let r: usize = ws.next().unwrap().parse().unwrap();
            (s, r)
        };
        cards.push(Card::new(s, r));
    }
    for c in cards.iter() {
        print!("{} ", c.to_string());
    }
    let hand = PokerHands::judge(&cards);
    println!("\n{}", &hand.to_string_ja());
}
