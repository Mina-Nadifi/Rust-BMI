#![allow(non_snake_case)]
use std::io;
#[derive(Debug)]
struct Client {
    name: String,
    weight: f32,
    height: f32,
    diet: String,
    condition: String,
}
impl Client {
    pub fn bmi(&self) -> f32 {
        //return self.weight / self.height.powf(2.0);
        self.weight * 10000.0 / (self.height * self.height)
    }
    /*fn longevity(&self) -> bool {
        if self.diet == "vegan" {
            true
        } else {
            false
        }
    }*/
    fn health(&mut self) {
        match self.bmi() {
            1.0..=18.5 => self.condition = "underweight range".to_string(),
            18.5..=24.9 => self.condition = "Healthy Weight range".to_string(),
            24.9..=29.9 => self.condition = "overweight range".to_string(),
            _ => self.condition = "error".to_string(),
        }
    }
    pub fn new() -> Self {
        Self {
            name: String::new(),
            weight: 0.0,
            height: 0.0,
            diet: String::new(),
            condition: String::new(),
        }
    }
}
fn main() {
    let mut members = String::new();
    let mut collection = Vec::new();
    println!("Number of Clients you have:");
    io::stdin()
        .read_line(&mut members)
        .expect("Entered members are invalid");
    let members: u8 = members.trim().parse().unwrap();
    for _i in 0..members {
        let mut babe = Client::new();
        let mut weight = String::new();
        println!("Please enter the client's name");
        io::stdin()
            .read_line(&mut babe.name)
            .expect("Entered name is invalid");
        println!("Please enter the client weight");
        io::stdin()
            .read_line(&mut weight)
            .expect("Entered weight is invalid");
        babe.weight = weight.trim().parse().unwrap();
        let mut height = String::new();
        println!("Please enter the client height");
        io::stdin()
            .read_line(&mut height)
            .expect("Entered height is invalid");
        babe.height = height.trim().parse().unwrap();
        println!("Please enter the client diet");
        io::stdin()
            .read_line(&mut babe.diet)
            .expect("Entered diet is invalid");
        babe.health();
        collection.push(babe);
    }
    println!("{:?}", collection);
}
