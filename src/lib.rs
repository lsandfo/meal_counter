use std::fs::{read_to_string, write};

/// get_save nimmt den Path zum Savefile als &str und gibt die enthaltenen Tage als i32 und die Kosten als f32 zurück.
/// Ist kein Savefile vorhanden werden 0 Tage un 0.00 Kosten zurück gegeben
fn get_save(save_file: &str) -> (i32, f32) {
    let sv_raw = read_to_string(save_file).unwrap_or("0;0.00".to_string());
    let mut sv = sv_raw.split(";");
    let days: i32 = sv.next().unwrap().parse::<i32>().unwrap();
    let value: f32 = sv.next().unwrap().parse::<f32>().unwrap();
    return (days, value);
}

/// write_save nimmt die Tage(days) als i32, Kosten(value) als f32 und den Path zum Savefile (save_file) als &str.
/// Es wird ein Result mit Ok(True) bei erfolg wiedergegeben, alternativ der entstandene Error
fn write_save(days: i32, value: f32, save_file: &str) -> Result<bool, Box<dyn std::error::Error>> {
    write(save_file, format!("{days};{value:.2}"))?;
    Ok(true)
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_get_save() {
        assert_eq!(get_save(""), (0, 0.00));
        assert_eq!(get_save("/home/lsandfo/.meal_counter"), (5, 12.50));
    }

    #[test]
    fn test_write_save() {
        let sv = "./tmp_save";
        write_save(5, 12.50, sv).unwrap();
        assert_eq!("5;12.50", read_to_string(sv).unwrap());
        std::fs::remove_file(sv).unwrap();
    }
}
