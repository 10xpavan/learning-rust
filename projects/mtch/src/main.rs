#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}                                  //lemme explain, youa re creating a enumaration for type o coins and for states, also you are eincudldnig, coin, and + states in coin enum, with quarter, also when the main fucntion runs, it calls valuecents, and telling it to ceck for usstate alaska, which and the coin associated to it was quarter, now inside match function it goes to quartersstate and prints teh satement.

fn valueincents(coin: Coin)-> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    valueincents(Coin::Quarter(UsState::Alaska));
}