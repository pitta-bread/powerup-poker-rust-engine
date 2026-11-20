use std::collections::HashMap;

use itertools::Itertools;
use rand::{rng, seq::SliceRandom};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, PartialOrd, Ord, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, PartialOrd, Ord, Hash)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn increase(self) -> Self {
        match self {
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King | Rank::Ace => Rank::Ace,
        }
    }

    pub fn decrease(self) -> Self {
        match self {
            Rank::Ace | Rank::King => Rank::King,
            Rank::Queen => Rank::Jack,
            Rank::Jack => Rank::Ten,
            Rank::Ten => Rank::Nine,
            Rank::Nine => Rank::Eight,
            Rank::Eight => Rank::Seven,
            Rank::Seven => Rank::Six,
            Rank::Six => Rank::Five,
            Rank::Five => Rank::Four,
            Rank::Four => Rank::Three,
            Rank::Three | Rank::Two => Rank::Two,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUp {
    IncreaseRank,
    DecreaseRank,
    AutoAce,
    PocketPair,
    ReDeal,
}

impl PowerUp {
    pub fn help_text(&self) -> &'static str {
        match self {
            PowerUp::IncreaseRank => "Increase the value of a card by +1. King goes to Ace.",
            PowerUp::DecreaseRank => "Decrease the value of a card by -1. Ace goes to King.",
            PowerUp::AutoAce => {
                "Auto-ace. Upgrades rank of one hole card of a choice to Ace (same suit)."
            }
            PowerUp::PocketPair => {
                "Pocket pair. Choose a hole card. \
                Swap the other card with a card from the \
                deck with the same value as the chosen card (but by necessity, \
                different suit, randomly found)."
            }
            PowerUp::ReDeal => {
                "Re-deal. Discard your current hole cards and draw \
                2 new ones from the top of the deck."
            }
        }
    }

    pub fn use_power(
        &self,
        deck: &mut Deck<PlayingCard>,
        hand: Hand,
        card_chosen_idx: Option<u8>,
    ) -> Result<Hand, &'static str> {
        // target correctly
        let (chosen_card, other_card) = match card_chosen_idx {
            Some(0) => (hand.playing_cards[0], hand.playing_cards[1]),
            Some(1) => (hand.playing_cards[1], hand.playing_cards[0]),
            Some(_) => return Err("hole card choice does not exist"),
            None => (hand.playing_cards[0], hand.playing_cards[1]),
        };

        match self {
            PowerUp::IncreaseRank => Ok(Hand {
                playing_cards: [
                    PlayingCard {
                        rank: chosen_card.rank.increase(),
                        suit: chosen_card.suit,
                    },
                    other_card,
                ],
                power_up: None,
            }),
            PowerUp::DecreaseRank => Ok(Hand {
                playing_cards: [
                    PlayingCard {
                        rank: chosen_card.rank.decrease(),
                        suit: chosen_card.suit,
                    },
                    other_card,
                ],
                power_up: None,
            }),
            PowerUp::AutoAce => Ok(Hand {
                playing_cards: [
                    PlayingCard {
                        rank: Rank::Ace,
                        suit: chosen_card.suit,
                    },
                    other_card,
                ],
                power_up: None,
            }),
            PowerUp::PocketPair => Ok(Hand {
                playing_cards: [
                    deck.draw_first_matching_ranked_card(chosen_card.rank)
                        .unwrap_or(other_card),
                    chosen_card,
                ],
                power_up: None,
            }),
            PowerUp::ReDeal => {
                if deck.cards.len() < 2 {
                    return Err("not enough cards to re-deal");
                }
                Ok(Hand {
                    playing_cards: [
                        deck.draw().ok_or("not enough cards to re-deal")?,
                        deck.draw().ok_or("not enough cards to re-deal")?,
                    ],
                    power_up: None,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayingCard {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Deck<T> {
    pub cards: Vec<T>,
}

impl<T> Deck<T> {
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<T> {
        self.cards.pop()
    }
}

impl Deck<PlayingCard> {
    pub fn new_52_card_deck() -> Self {
        // shuffles too
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(PlayingCard { rank, suit });
            }
        }
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }

    pub fn draw_first_matching_ranked_card(
        &mut self,
        rank: Rank,
    ) -> Result<PlayingCard, &'static str> {
        let mut cards_to_go_back_on_top = Vec::<PlayingCard>::new();
        let mut card = self.cards.pop();
        if card.is_none() {
            return Err("deck cannot be empty at start");
        }
        while !card.is_none() && card.unwrap().rank != rank {
            cards_to_go_back_on_top.push(card.unwrap());
            card = self.cards.pop();
        }
        self.cards.extend(cards_to_go_back_on_top.into_iter().rev());
        card.ok_or("deck cannot be empty before finding desired card")
    }
}

impl Deck<PowerUp> {
    pub fn new_power_up_deck(
        increase_rank_count: usize,
        decrease_rank_count: usize,
        auto_ace_count: usize,
        pocket_pair_count: usize,
        re_deal_count: usize,
    ) -> Self {
        // shuffles too
        let mut cards = Vec::new();
        cards.extend((0..increase_rank_count).map(|_| PowerUp::IncreaseRank));
        cards.extend((0..decrease_rank_count).map(|_| PowerUp::DecreaseRank));
        cards.extend((0..auto_ace_count).map(|_| PowerUp::AutoAce));
        cards.extend((0..pocket_pair_count).map(|_| PowerUp::PocketPair));
        cards.extend((0..re_deal_count).map(|_| PowerUp::ReDeal));
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    pub playing_cards: [PlayingCard; 2],
    pub power_up: Option<PowerUp>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub chips: usize,
    pub hand: Option<Hand>,
    pub current_bet: usize,
    pub last_action: Option<Action>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    NotStarted,
    PreFlop,
    Flop,
    Showdown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    pub pot: usize,
    pub community_cards: Option<Vec<PlayingCard>>,
    /// Power-ups played by each player in the current or most recent hand.
    pub played_power_ups: [Option<PowerUp>; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowdownResult {
    Winner(usize),
    SplitPot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    pub players: [Player; 2],
    pub round: usize,
    pub phase: Phase,
    pub acts_first_this_hand: usize, // 0 or 1
    pub active_player: usize,        // 0 or 1
    pub card_deck: Deck<PlayingCard>,
    pub power_up_deck: Deck<PowerUp>,
    pub table: Table,
}

impl GameState {
    pub fn setup_new_game(player_1_name: &str, player_2_name: &str) -> Self {
        GameState {
            players: [
                Player {
                    name: player_1_name.to_string(),
                    chips: 15,
                    hand: None,
                    current_bet: 0,
                    last_action: None,
                },
                Player {
                    name: player_2_name.to_string(),
                    chips: 15,
                    hand: None,
                    current_bet: 0,
                    last_action: None,
                },
            ],
            round: 0,
            phase: Phase::NotStarted,
            acts_first_this_hand: 1,
            active_player: 0,
            card_deck: Deck::new_52_card_deck(),
            power_up_deck: Deck::new_power_up_deck(5, 5, 2, 2, 6),
            table: Table {
                pot: 0,
                community_cards: None,
                played_power_ups: [None; 2],
            },
        }
    }

    pub fn start_hand(&mut self) -> Result<(), &'static str> {
        // increments
        self.round += 1;
        self.phase = Phase::PreFlop;
        self.acts_first_this_hand = 1 - self.acts_first_this_hand;
        self.active_player = self.acts_first_this_hand;
        self.table = Table {
            pot: 0,
            community_cards: None,
            played_power_ups: [None; 2],
        };

        // shuffle decks
        self.card_deck.shuffle();
        self.power_up_deck.shuffle();

        // hands and antes
        for player in &mut self.players {
            let c1 = self.card_deck.draw().ok_or("Playing card deck empty!")?;
            let c2 = self.card_deck.draw().ok_or("Playing card deck empty!")?;
            player.hand = Some(Hand {
                playing_cards: [c1, c2],
                power_up: self.power_up_deck.draw(),
            });
            if player.chips == 0 {
                return Err("Player has no chips for required ante.");
            }
            player.chips -= 1;
            self.table.pot += 1;
        }

        Ok(())
    }

    pub fn clean_up(&mut self, result: ShowdownResult) {
        self.phase = Phase::NotStarted;
        self.card_deck = Deck::new_52_card_deck();
        self.power_up_deck = Deck::new_power_up_deck(5, 5, 2, 2, 6);

        match result {
            ShowdownResult::SplitPot => {
                self.players[0].chips += self.table.pot / 2;
                self.players[1].chips += self.table.pot / 2;
            }
            ShowdownResult::Winner(winner_idx) => {
                self.players[winner_idx].chips += self.table.pot;
            }
        }
        self.table.pot = 0;

        for player in &mut self.players {
            player.hand = None;
        }
    }

    pub fn step_turn(
        &mut self,
        player_id: usize,
        action: Action,
        power_up_use: Option<PowerUpUse>,
    ) -> Result<TurnOutcome, &'static str> {
        fn clean_up_players_ready_for_next_phase(state: &mut GameState) {
            // clear betting
            for player in &mut state.players {
                player.current_bet = 0;
                player.last_action = None;
            }
        }

        // VALIDATIONS
        if !(player_id == 0 || player_id == 1) {
            return Err("player id does not exist");
        }
        if player_id != self.active_player {
            return Err("not this player's turn!");
        }
        if !matches!(self.phase, Phase::PreFlop | Phase::Flop) {
            return Err("can't take a turn outside a betting phase");
        }
        if power_up_use.is_some() && self.phase != Phase::Flop {
            return Err("power-ups can only be used post-flop");
        }
        if power_up_use.is_some() && action == Action::Fold {
            return Err("power-ups cannot be used when folding");
        }

        let (first, second) = self.players.split_at_mut(1);
        let (this_player, other_player) = if player_id == 1 {
            (&mut second[0], &mut first[0])
        } else {
            (&mut first[0], &mut second[0])
        };

        if let Action::Bet(bet_size) = action {
            if !(1..=2).contains(&bet_size) {
                return Err("bet must be 1 or 2 chips");
            }
            if this_player.chips == 0 {
                return Err("no chips left to bet");
            }
            if other_player.current_bet > 0 {
                return Err("raising is not allowed");
            }
        }
        if action == Action::CheckOrCall
            && other_player.current_bet == 0
            && other_player.last_action.is_some()
            && other_player.last_action != Some(Action::CheckOrCall)
        {
            return Err("impossible situation on check/call, based on other player");
        }

        // POWER UPS
        if let Some(power_up_use) = power_up_use {
            let hand = this_player.hand.ok_or("player has no hand")?;
            let power_up = hand.power_up.ok_or("player has no power-up")?;
            match (power_up, power_up_use.card_chosen_idx) {
                (PowerUp::ReDeal, None) => {}
                (PowerUp::ReDeal, Some(_)) => return Err("re-deal does not need a card choice"),
                (_, Some(0 | 1)) => {}
                (_, Some(_)) => return Err("hole card choice does not exist"),
                (_, None) => return Err("choose a hole card for the power-up"),
            }
            this_player.hand = Some(power_up.use_power(
                &mut self.card_deck,
                hand,
                power_up_use.card_chosen_idx,
            )?);
            self.table.played_power_ups[player_id] = Some(power_up);
        }

        this_player.last_action = Some(action);

        // OUTCOME
        match action {
            Action::Fold => {
                other_player.chips += other_player.current_bet;
                let other_player_idx: usize = if player_id == 1 { 0 } else { 1 };

                clean_up_players_ready_for_next_phase(self);
                self.clean_up(ShowdownResult::Winner(other_player_idx));
                Ok(TurnOutcome::NextHand)
            }
            Action::CheckOrCall => {
                if other_player.current_bet > 0 {
                    if other_player.current_bet > this_player.chips {
                        // all in
                        let call_amount = this_player.chips;
                        let uncalled = other_player.current_bet - call_amount;
                        other_player.chips += uncalled;
                        other_player.current_bet = call_amount;

                        this_player.current_bet = call_amount;
                        this_player.chips = 0;
                    } else {
                        this_player.chips -= other_player.current_bet;
                        this_player.current_bet = other_player.current_bet;
                    }

                    self.table.pot += this_player.current_bet + other_player.current_bet;

                    clean_up_players_ready_for_next_phase(self);
                    Ok(TurnOutcome::NextPhase)
                } else if other_player.last_action == Some(Action::CheckOrCall) {
                    self.table.pot += this_player.current_bet + other_player.current_bet;

                    clean_up_players_ready_for_next_phase(self);
                    Ok(TurnOutcome::NextPhase)
                } else if other_player.last_action.is_none() {
                    self.active_player = 1 - self.active_player;
                    Ok(TurnOutcome::NextPlayer)
                } else {
                    Err("impossible situation on check/call, based on other player")
                }
            }
            Action::Bet(bet_size) => {
                if bet_size as usize > this_player.chips {
                    // all in
                    this_player.current_bet = this_player.chips;
                    this_player.chips -= this_player.chips;
                } else {
                    this_player.chips -= bet_size as usize;
                    this_player.current_bet = bet_size as usize;
                }

                self.active_player = 1 - self.active_player;
                Ok(TurnOutcome::NextPlayer)
            }
        }
    }

    pub fn advance_phase(&mut self) -> Result<(), &'static str> {
        // ensure first to act is next up
        self.active_player = self.acts_first_this_hand;

        match self.phase {
            Phase::PreFlop => {
                // deal 3 cards
                let cards = vec![
                    self.card_deck
                        .draw()
                        .ok_or("deck empty when drawing flop 1")?,
                    self.card_deck
                        .draw()
                        .ok_or("deck empty when drawing flop 2")?,
                    self.card_deck
                        .draw()
                        .ok_or("deck empty when drawing flop 3")?,
                ];
                self.table.community_cards = Some(cards);
                self.phase = Phase::Flop;
                Ok(())
            }
            Phase::Flop => {
                // deal 2 cards
                self.table
                    .community_cards
                    .as_mut()
                    .ok_or("missing community cards")?
                    .extend([
                        self.card_deck
                            .draw()
                            .ok_or("deck empty when drawing turn")?,
                        self.card_deck
                            .draw()
                            .ok_or("deck empty when drawing river")?,
                    ]);
                self.phase = Phase::Showdown;
                Ok(())
            }
            Phase::Showdown => {
                self.resolve_showdown()?;
                Ok(())
            }
            _ => Err("Not an advancable phase."),
        }
    }

    pub fn resolve_showdown(&mut self) -> Result<ShowdownResult, &'static str> {
        let mut player_1_hand: Vec<PlayingCard> =
            self.players[0].hand.unwrap().playing_cards.to_vec();
        let mut table: Vec<PlayingCard> = self
            .table
            .community_cards
            .clone()
            .ok_or("no cards on table")?;
        player_1_hand.append(&mut table);

        let mut player_2_hand: Vec<PlayingCard> =
            self.players[1].hand.unwrap().playing_cards.to_vec();
        let mut table: Vec<PlayingCard> = self
            .table
            .community_cards
            .clone()
            .ok_or("no cards on table")?;
        player_2_hand.append(&mut table);

        let player_1_hand_rank = evaluate_hand(&player_1_hand)?;
        let player_2_hand_rank = evaluate_hand(&player_2_hand)?;

        if player_1_hand_rank == player_2_hand_rank {
            self.clean_up(ShowdownResult::SplitPot);
            Ok(ShowdownResult::SplitPot)
        } else if player_1_hand_rank > player_2_hand_rank {
            self.clean_up(ShowdownResult::Winner(0));
            Ok(ShowdownResult::Winner(0))
        } else {
            self.clean_up(ShowdownResult::Winner(1));
            Ok(ShowdownResult::Winner(1))
        }
    }

    pub fn game_winner(&self) -> Option<usize> {
        if self.players[0].chips == 0 {
            Some(1)
        } else if self.players[1].chips == 0 {
            Some(0)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Fold,
    CheckOrCall,
    Bet(u8), // only 1 or 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerUpUse {
    /// The chosen hole-card index, or `None` for ReDeal.
    pub card_chosen_idx: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnOutcome {
    NextPlayer,
    NextPhase,
    NextHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard([u8; 5]),     // all ranks biggest to smallest
    Pair([u8; 4]),         // pair rank, kickers
    TwoPair([u8; 3]),      // top pair rank, second pair rank, kicker rank
    ThreeOfAKind([u8; 3]), // trips rank, first kicker, second kicker
    Straight(u8),          // rank of top of straight
    Flush([u8; 5]),        // ordered ranks top to bottom
    FullHouse([u8; 2]),    // trips rank, pair rank
    FourOfAKind([u8; 2]),  // quads rank, kicker rank
    StraightFlush(u8),     // rank of top of straight
    FiveOfAKind(u8),       // quints rank
}

pub fn rank_hand(cards: [PlayingCard; 5]) -> HandRank {
    let mut ordered_cards = cards;
    ordered_cards.sort();
    let first_card_rank = ordered_cards.first().unwrap().rank;
    let first_card_suit = ordered_cards.first().unwrap().suit;
    let last_card_rank = ordered_cards.last().unwrap().rank;
    let ranks_only: [u8; 5] = ordered_cards.map(|x| x.rank as u8);

    // to avoid panic below, if 5 of a kind, return early here
    if ranks_only.iter().all_equal() {
        return HandRank::FiveOfAKind(ranks_only[0]);
    }

    // counts of ranks for pairs up
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for x in ranks_only {
        *counts.entry(x).or_default() += 1;
    }
    let (max_rank, max_count) = counts
        .iter()
        .max_by_key(|element| element.1)
        .map(|(rank, count)| (*rank, *count))
        .unwrap();
    counts.remove(&max_rank);
    let (second_max_rank, second_max_count) = counts
        .iter()
        .max_by_key(|element| element.1)
        .map(|(rank, count)| (*rank, *count))
        .unwrap();

    // flush checker
    let mut flush = true;
    for card in ordered_cards {
        if first_card_suit != card.suit {
            flush = false;
            break;
        }
    }

    // straight checker
    let mut straight = true;
    let ace_low_straight = last_card_rank == Rank::Ace
        && ranks_only[0] == 2
        && ranks_only[1] == 3
        && ranks_only[2] == 4
        && ranks_only[3] == 5;
    if !ace_low_straight {
        for (idx, card) in ordered_cards.iter().enumerate() {
            if (first_card_rank as u8) + (idx as u8) != (card.rank as u8) {
                straight = false;
                break;
            }
        }
    }

    // always get general kickers, won't work for two pair
    let mut kickers = ranks_only.to_vec();
    kickers.retain(|&x| x != max_rank);

    let straight_high = if ace_low_straight {
        Rank::Five as u8
    } else {
        last_card_rank as u8
    };

    // return result
    if straight && flush {
        HandRank::StraightFlush(straight_high)
    } else if max_count == 4 {
        HandRank::FourOfAKind([max_rank, second_max_rank])
    } else if max_count == 3 && second_max_count == 2 {
        HandRank::FullHouse([max_rank, second_max_rank])
    } else if flush {
        let mut ranks = ranks_only;
        ranks.reverse();
        HandRank::Flush(ranks)
    } else if straight {
        HandRank::Straight(straight_high)
    } else if max_count == 3 {
        HandRank::ThreeOfAKind([
            max_rank,
            *kickers.iter().max().unwrap(),
            *kickers.iter().min().unwrap(),
        ])
    } else if max_count == 2 && second_max_count == 2 {
        let mut both_pairs = [max_rank, second_max_rank];
        both_pairs.sort();
        let mut last_card = ranks_only.to_vec();
        last_card.retain(|&x| x != max_rank && x != second_max_rank);
        HandRank::TwoPair([
            *both_pairs.last().unwrap(),
            *both_pairs.first().unwrap(),
            last_card[0],
        ])
    } else if max_count == 2 {
        HandRank::Pair([max_rank, kickers[2], kickers[1], kickers[0]])
    } else {
        let mut ranks = ranks_only;
        ranks.reverse();
        HandRank::HighCard(ranks)
    }
}

pub fn evaluate_hand(hand_plus_community: &[PlayingCard]) -> Result<HandRank, &'static str> {
    let all_combinations_of_5: Vec<Vec<PlayingCard>> = {
        hand_plus_community
            .iter()
            .copied()
            .combinations(5)
            .collect()
    };
    let mut ranked_hands = HashMap::new();
    for hand in all_combinations_of_5 {
        let hand_rank = rank_hand(*hand.as_array().unwrap());
        ranked_hands.insert(hand, hand_rank);
    }
    ranked_hands
        .values()
        .copied()
        .max()
        .ok_or("must have at least one hand to rank")
}
