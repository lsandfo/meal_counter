use meal_counter::{get_save, print_save, write_save};
use std::{env, process::exit};

fn main() {
    const PRICE: f32 = 2.50;
    let mut days: i32;
    let value: f32;

    let home = env::var("HOME").expect("HOME var missing!");
    let save_file_buffer = format!("{home}/.meal_counter");
    let save_file = save_file_buffer.as_str();
    let args: Vec<String> =  env::args().collect();
    if args.len() == 1 {
        (days, _) = get_save(save_file);
        days += 1;
        value = days as f32 * PRICE;
        print_save(days, value);
        write_save(days, value, save_file).unwrap();
    } else if args.len() >= 2 {
        
        match args[1].as_str() {
            "-m" => {
                (days, _) = get_save(save_file);
                days -= 1;
                value = days as f32 * PRICE;
                print_save(days, value);
                write_save(days, value, save_file).unwrap();
            }
            "-s" => {
                if args.len() >= 3 {
                    days = args[2]
                        .parse::<i32>()
                        .expect("-s braucht Zahl als arugment!");
                    value = days as f32 * PRICE;
                    print_save(days, value);
                    write_save(days, value, save_file).unwrap();
                } else {
                    println!("-s braucht Zahl als Argument!");
                    exit(0);
                }
            }
            "-i" => {
                (days, value) = get_save(save_file);
                print_save(days, value);
            }
            "-h" | _ => println!(
                "Options:\nnone\tTag Incrementieren\n-h\thelp\n-m\tMinus 1 Tag\n-s x\tAuf Tag x setzen\n-i\tTage anzeigen"
            ),
        }
    } else {
        println!("Da ist etwas schief gelaufen!");
    }
}
