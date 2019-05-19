use std::cell::Cell;
use rand::Rng;

//id: 0. player 1. dealer
struct Person {
    id: i8,
    cards_mark: Cell<[i8; 5]>,
    cards_value: Cell<[i8; 5]>,
    num: Cell<i8>,
}

fn random_card(given_cards: &mut Vec<i32>) -> i32{
    let mut rng = rand::thread_rng();
    loop {
        let mark: i32 = rng.gen_range(0,4);
        let value: i32= rng.gen_range(1,13);
        let mut not_exist_generated_value = true;
        let res = (mark + value*10) as i32;
        for i in given_cards.iter() {
            if *i == res{
                not_exist_generated_value = false;
                break;
            }
        }
        if not_exist_generated_value{            
            return res;
        }
    }
}

fn draw(drawer: &mut Person, given_cards: &mut Vec<i32>){
    let mut marks: [i8; 5] = drawer.cards_mark.get();
    let mut value: [i8; 5] = drawer.cards_value.get();

    let res = random_card(given_cards);
    (*given_cards).push(res);
    let x1: i8 = (res / 10) as i8;
    let x2: i8 = (res % 10) as i8;

    value[drawer.num.get() as usize] = x1;
    marks[drawer.num.get() as usize] = x2;

    drawer.cards_mark.set(marks);
    drawer.cards_value.set(value);
    drawer.num.set(drawer.num.get()+1);
}

fn initialize(player: &mut Person, dealer: &mut Person, given_cards: &mut Vec<i32>){
    draw(player, given_cards);
    draw(player, given_cards);
    draw(dealer, given_cards);
    draw(dealer, given_cards);
}

fn check_mark(num: i8) -> &'static str{
    if num == 0{
        return "heart"
    }
    else if num == 1{
        return "clover"
    }
    else if num == 2{
        return "spade"
    }
    else {
        return "dia"
    }
}

fn print_field(player: &mut Person, dealer: &mut Person, given_cards: &mut Vec<i32>){
    println!("------------");
    println!("Player Field");
    let mut player_sum: i32 = 0;
    for i in 0..player.num.get(){
        let index: usize = i as usize;
        print!("M:{} V:{} ",check_mark(player.cards_mark.get()[index]), player.cards_value.get()[index]);
        player_sum += (player.cards_value.get()[index]) as i32;
    }
    println!("");
    println!("Sum {}", player_sum);
    println!("------------");
    println!("Dealer Field");
    let mut dealer_sum: i32 = 0;
    for i in 0..dealer.num.get(){
        let index: usize = i as usize;
        print!("M:{} V:{} ",check_mark(dealer.cards_mark.get()[index]), dealer.cards_value.get()[index]);   
        dealer_sum += (dealer.cards_value.get()[index]) as i32;   
    }
    println!("");
    println!("Sum {}", dealer_sum);
    println!("------------");

    if dealer_sum < 16 && dealer.num.get() < 5{
        draw( dealer, given_cards);
        print_field( player, dealer, given_cards);
        return;
    }

    if player_sum < 16 && player.num.get() < 5{
        draw(player, given_cards);
        print_field( player, dealer, given_cards);
        return;
    }

    if (player_sum == dealer_sum) || (player_sum > 21 && dealer_sum > 21){    
        println!("result: draw");
    } else if (dealer_sum > player_sum && dealer_sum <= 21) || (dealer_sum < player_sum && player_sum > 21) {
        println!("result: dealer win");
    } else {
        println!("result: player win");
    }
    println!("");
    println!("");
    println!("");
}

fn main() {    
    let mut player = Person { id: 0, cards_mark: Cell::new([0,0,0,0,0]), cards_value: Cell::new([0,0,0,0,0]), num: Cell::new(0) };
    let mut dealer = Person { id: 1, cards_mark: Cell::new([0,0,0,0,0]), cards_value: Cell::new([0,0,0,0,0]), num: Cell::new(0)  };
    //0: heart, 1: clover, 2:spade, 3: dia    
    let cards_mark_template = [0,1,2,3];
    let cards_value_template = [1,2,3,4,5,6,7,8,9,10,11,12,13];

    let mut given_cards: Vec<i32> = Vec::with_capacity(cards_mark_template.len()*cards_value_template.len());
    println!("Black Jack!");
    loop {
        initialize(&mut player,&mut dealer,&mut given_cards);
        print_field(&mut player,&mut dealer,&mut given_cards);
        given_cards.clear();
        player.num.set(0);
        dealer.num.set(0);
    }

}
