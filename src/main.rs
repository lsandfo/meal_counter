use std::fs::read_to_string;

fn get_save(save_file: &str) -> (i32, f32) {
    let sv_raw = read_to_string(save_file).unwrap_or("0;0.00".to_string());
    let mut sv = sv_raw.split(";");
    let days: i32 = sv.next().unwrap().parse::<i32>().unwrap();
    let value: f32 = sv.next().unwrap().parse::<f32>().unwrap();
    return (days, value);
}

fn main() {}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_get_save() {
        assert_eq!(get_save(""), (0, 0.00));
        assert_eq!(get_save("/home/lsandfo/.meal_counter"), (5, 12.50));
    }
}
