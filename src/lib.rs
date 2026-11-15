//! # meal_counter
//! Zählt Tage und berechnet pro Tag die gesammt Kosten
//! 
//! ## Usage:
//! Options:                            <br>
//! none    Tag Incrementieren          <br>
//! -a      Tag manuell Incrementieren  <br> 
//! -h      help                        <br>
//! -m      Minus 1 Tag                 <br>
//! -s x    Auf Tag x setzen            <br>
//! -i      Tage anzeigen               <br>
//! -r      Tage auf 0 zurück setzen    <br>
//! 
//! ## Kosten Info:
//! Die Kosten (value) sind Hard auf 2.50 Codiert (Siehe Lib.rs => PRICE) und werden mit der gegebenen Tagesanzahl (days) multiplizert.
//! 
//! ## Tag erhöhung
//! Ohne Argument, wird der Tag bei ausführung einmal täglich um einen Tag erhöht.
//! Möchte man den Tag Counter dann nochmal verändern, muss dies per Argument geschehen.


use std::fs::{read_to_string, write};
use chrono::prelude::*;

/// get_save nimmt den Path zum Savefile als &str und gibt die enthaltenen Tage als i32, die Kosten als f32
/// und das aktuelle Datum YYYY.mm.dd als String zurück.
/// Ist kein Savefile vorhanden werden 0 Tage, 0.00 Kosten und 0000.00.00 als Datum zurück gegeben
pub fn get_save(save_file: &str) -> (i32, f32, String) {
    let sv_raw = read_to_string(save_file).unwrap_or(format!("0;0.00;{}", get_date()));
    let mut sv = sv_raw.split(";");
    let days: i32 = sv.next().unwrap().parse::<i32>().unwrap_or(0);
    let value: f32 = sv.next().unwrap().parse::<f32>().unwrap_or(0.00);
    let date: String = sv.next().unwrap_or(&get_date()).to_string();
    return (days, value, date);
}

/// write_save nimmt die Tage(days) als i32, Kosten(value) als f32, Datum(date) als String 
/// und den Path zum Savefile (save_file) als &str.
/// Es wird ein Result mit Ok(True) bei erfolg wiedergegeben, alternativ der entstandene Error
pub fn write_save(
    days: i32,
    value: f32,
    save_file: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let date = get_date();
    write(save_file, format!("{days};{value:.2};{date}"))?;
    Ok(true)
}

/// print_save printetd einfach die gegebennen Tage(days, i32), Kosten(value, f32) und Datum(date, String).
pub fn print_save(days: i32, value: f32, date: &String) {
    println!("Tage:\t{days:>5}\nKosten:\t{value:>5.2}\nStand:\t{date}");
}

/// get_date ist Wrapper für Aktuelles Datum im richtigen format,
/// dass als String returned wird.
pub fn get_date() -> String{
    Local::now().format("%Y.%m.%d").to_string()
}

#[cfg(test)]
mod testing {
    use std::fs::remove_file;

use super::*;

    #[test]
    fn test_get_save() {
        let test_file: &str = "./test_save";
        write(test_file, "5;12.50;2020.08.04").unwrap();
        assert_eq!(get_save(""), (0, 0.00, get_date()));
        assert_eq!(get_save(test_file), (5, 12.50, "2020.08.04".to_string()));
        remove_file(test_file).unwrap();
    }

    #[test]
    fn test_write_save() {
        let date = get_date();
        let sv = "./tmp_save";
        write_save(5, 12.50, sv).unwrap();
        assert_eq!(format!("5;12.50;{date}"), read_to_string(sv).unwrap());
        std::fs::remove_file(sv).unwrap();
    }

    #[test]
    fn test_print_save() {
        print_save(5, 12.50, &"2026.11.14".to_string());
        print_save(31, 77.50, &"9999.12.31".to_string());
        print_save(0, 0.00, &"0000.00.00".to_string());
    }
}
