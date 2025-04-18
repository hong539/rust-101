// Convert temperatures between Fahrenheit and Celsius.
use std::io;
use std::str::FromStr;

/// 轉換溫度的函數（要求格式如 25.7°C 或 77.9°F）
fn convert_temperature(input: &str) -> Result<String, &'static str> {
    let input = input.trim();

    if input.len() < 3 {
        return Err("輸入格式錯誤！請使用格式 25.7°C 或 77.9°F。");
    }

    // **Unicode 安全解析**
    let mut chars = input.char_indices().rev();
    let (_, last_char) = chars.next().unwrap_or((0, ' ')); // 取得最後一個字元
    let (second_last_idx, second_last_char) = chars.next().unwrap_or((0, ' ')); // 取得倒數第二個字元

    let unit = match (second_last_char, last_char) {
        ('°', 'C') => 'C',
        ('°', 'F') => 'F',
        _ => return Err("溫度單位錯誤！請使用 '°C' 或 '°F'。"),
    };

    let value_part = &input[..second_last_idx].trim(); // 切割掉最後兩個字元

    let value = match f64::from_str(value_part) {
        Ok(v) => v,
        Err(_) => return Err("數值解析錯誤！請確保格式為 25.7°C 或 77.9°F。"),
    };

    // // **新增範圍檢查**
    // if !( -1000.0..=1000.0 ).contains(&value) {
    //     return Err("溫度超出範圍！請輸入 -1000°C ~ 1000°C 之間的值。");
    // }

    let result = if unit == 'C' {
        format!("{:.2}°F", (value * 9.0 / 5.0) + 32.0)
    } else {
        format!("{:.2}°C", (value - 32.0) * 5.0 / 9.0)
    };

    Ok(result)
}

// Generate the nth Fibonacci number.
fn fibonacci_iterative(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
];

const GIFTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn print_lyrics() {
    for day in 0..12 {
        println!("\nOn the {} day of Christmas my true love sent to me:", DAYS[day]);

        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                println!("And {}", GIFTS[gift].to_lowercase());
            } else {
                println!("{}", GIFTS[gift]);
            }
        }
    }
}

fn main() {
    println!("請輸入溫度（格式：25.7°C 或 77.9°F）：");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("讀取輸入失敗");

    match convert_temperature(&input) {
        Ok(result) => println!("轉換結果：{}", result),
        Err(err) => println!("錯誤：{}", err),
    }

    let n = 50;
    println!("The {}th Fibonacci number is: {}", n, fibonacci_iterative(n));

    print_lyrics();    
}