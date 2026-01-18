use std :: io;

fn main(){
    println!("🌡️Temprature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsuis");
    println!("PLease select an option (1 or 2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!(" Invalid choice. Please enter 1 or 2.");
            return;
        }
    };
    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid Choice");
    }
}

fn celsius_to_fahrenheit(){
    println!("Enter temprature in Celsius:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    let fahrenheit = (temp * 9.0/5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", temp ,fahrenheit);
}

fn fahrenheit_to_celsius(){
    println!("ENter temparture in Fahrenheit");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) =>{
            println!("Invalid input");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 /9.0;
    println!("{:.2}°F is {:.2}°C", temp , celsius);
}