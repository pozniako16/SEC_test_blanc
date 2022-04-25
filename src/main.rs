use std::fmt;
use std::fmt::Formatter;
use read_input::prelude::*;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

#[macro_use]
extern crate strum_macros;

fn main() {
    println!("Hello, world!");
    let mut nbr_pizz_cheese = 0;
    let mut pizza_wanted = 0;
    let mut pizza_vect:Vec<Pizza> = Vec::new();
    match input::<u32>().repeat_msg("How many pizza do you want ?").get() {
        0 => {
            println!("Why do you bother ? Get a life ...");
            return
        }
        nbr => {
            pizza_wanted = nbr;
        }
    }
    let mut i = 0;
    loop {
        let mut wrong_input = false;
        let mut display_mess = format!("Enter the type for pizza {} {}: ", i+1, get_all_pizza_str());
        match input::<String>().repeat_msg( display_mess).get().as_str() {
            "Margarita" => {
                pizza_vect.push(Pizza{
                    pizza_type: PizzaType::Margarita,
                    has_cheese: true,
                    price: 7
                })
            }
            "Marinara" => {
                pizza_vect.push(Pizza{
                    pizza_type: PizzaType::Marinara,
                    has_cheese: true,
                    price: 5
                })
            }
            "Royal" => {
                pizza_vect.push(Pizza{
                    pizza_type: PizzaType::Royal,
                    has_cheese: true,
                    price: 10
                })
            }
            _ => {
                wrong_input = true;
                println!("Wrong input please retype")
            }
        }
        if !wrong_input {
            i += 1;
            if i >= pizza_wanted {
                break;
            }
        }
    }
    let mut total_price = 0;
    for pizza in pizza_vect.iter() {
        total_price += pizza.price;
        if pizza.has_cheese {
            nbr_pizz_cheese += 1
        }
    }
    println!("The total is  {}", total_price);
    let mut email_addr;
    let mut credit_card_nbr;
    let mut card_date;
    match input::<String>().repeat_msg("Please enter your email address for delivery: ").get() {
        email => {email_addr = email}
    }
    loop {
        match input::<String>().repeat_msg("Please enter credit card number").get() {
            card_nbr => {
                if card_validate(&card_nbr) {
                    credit_card_nbr = card_nbr;
                    break
                } else {
                    println!("Enter valid card nbr")
                }
            }
        }
    }
    match input::<String>().repeat_msg("Enter the credit card validity in the MM//YY format: ").get() {
        date => {card_date = date}
    }
    println!("*******************");
    println!("Debiting {} CHF from card {} with validity {}", total_price, credit_card_nbr, card_date);
    println!("Sending {} to {}", get_vector_pizza_str(&pizza_vect), email_addr);
    println!("{} pizzas have cheese", nbr_pizz_cheese);
}

struct Pizza {
    pizza_type: PizzaType,
    has_cheese: bool,
    price: u32
}

#[derive(EnumIter)]
enum PizzaType {
    #[strum(serialize="Margarita")]
    Margarita,
    #[strum(serialize="Marinara")]
    Marinara,
    #[strum(serialize="Royal")]
    Royal
}

impl fmt::Display for PizzaType{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PizzaType::Margarita => {write!(f, "Margarita")}
            PizzaType::Marinara => {write!(f, "Marinara")}
            PizzaType::Royal => {write!(f, "Royal")}
        }
    }
}

fn get_all_pizza_str() -> String {
    let mut output_string = "[".to_string();
    let mut i = 0;
    let mut total_pizz = PizzaType::iter().count();
    for pizz_type in PizzaType::iter(){
        output_string = format!("{}{}", output_string, pizz_type);
        i += 1;
        if i < total_pizz {
            output_string = format!("{},", output_string);
        }
    }
    output_string = format!("{}]", output_string);
    output_string
}

fn get_vector_pizza_str(pizzas: &Vec<Pizza>) -> String {
    let mut output_string = "[".to_string();
    let mut i = 0;
    let mut total_pizz = pizzas.len();
    for pizz in pizzas.iter() {
        output_string = format!("{}{}", output_string, pizz.pizza_type);
        i += 1;
        if i < total_pizz {
            output_string = format!("{},", output_string);
        }
    }
    output_string = format!("{}]", output_string);
    output_string
}

extern crate card_validate;
use card_validate::Validate;

fn validate_credit_card(card_nbr: &str) -> bool {
    return match Validate::from(card_nbr) {
        Ok(result) => false,
        Err(err) => true
    }
}

fn validate_date(date: &str) -> bool {
    
}

