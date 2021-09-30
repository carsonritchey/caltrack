use std::{fs, io::{prelude::*, BufReader, Write}};
use chrono;

const DAT_DIR: &str = "dat/";

const MEAL_LOG_NAME: &str = "meal_log.txt";
const USER_INFO_NAME: &str = "user_info.txt";

const SEPERATOR: char = ',';

fn main() {
    // puts all previously eaten meals in one place
    let mut menu: Vec<Meal> = Meal::file_to_vec();

    loop {
        main_menu(&mut menu);
    }
}

// "main menu" options that user can choose 
enum MainOptions {
    Eat,
    Stats,
    Log,
    Exit
}
impl MainOptions {
    fn value(&self) -> &str {
        match *self {
            MainOptions::Eat   => "eat",
            MainOptions::Stats => "stats",
            MainOptions::Log   => "log",
            MainOptions::Exit  => "exit",
        }
    }
}

fn main_menu(menu: &mut Vec<Meal>) {
    println!("\ntoday you've eaten {} calories", todays_calories(menu));
    loop {
        let s = prompt("");

        if s == MainOptions::Eat.value() {
            println!("eating");
            eat(menu);
        } else if s == MainOptions::Stats.value() {
            println!("statting");
            stats(menu);
        } else if s == MainOptions::Log.value() {
            log(menu);
        } else if s == MainOptions::Exit.value() {
            std::process::exit(1);
        } else {
            continue;
        }

        break;
    }
}

struct Meal {
    calories: usize,
    date: String,
}
impl Meal {
    // returns a vector of meals taken from the meal log file
    fn file_to_vec() -> Vec<Meal> {
        let mut meals: Vec<Meal> = Vec::new();

        let f = fs::File::open(format!("{}{}", DAT_DIR, MEAL_LOG_NAME)).expect("unable to find meal log file");
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let split: Vec<String> = line.unwrap().to_string().split(SEPERATOR).map(|s| s.to_string()).collect();
            meals.push(Meal {calories: split[1].parse::<usize>().expect("unable to parse meal log data"), date: split.get(0).expect("no date found in meal log").to_string()});
        }

        return meals;
    }

    // writes meal vector to meal log file
    fn vec_to_file(menu: &Vec<Meal>) {
        // clears meal log file
        fs::OpenOptions::new().write(true).truncate(true).open(format!("{}{}", DAT_DIR, MEAL_LOG_NAME)).expect("unable to clear meal log file");

        // writes all meals to file
        for meal in menu {
            Meal::write_meal(meal);
        }
    }

    // appends meal to meal log 
    fn write_meal(&self) {
        let mut f = fs::OpenOptions::new().write(true).append(true).open("dat/meal_log.txt").expect("unable to open meal log file");

        f.write_all(format!("{}{}{}\n", self.date, SEPERATOR, self.calories).as_bytes()).expect("unable to write to meal log file");
    }

    // removes index from menu and writes it to meal log
    fn remove_index(index: usize, menu: &mut Vec<Meal>) {
        menu.remove(index - 1);
        Meal::vec_to_file(menu);
    }
}

fn new_meal() -> Meal {
    Meal {
        calories: prompt_and_parse("calories in meal?"),
        date: get_date_string(),
    }
}

// assumes day by day basis
struct UserInfo {
   bmr: usize, // basal metabolic rate
   goal: usize, // desired max calorie intake
}
impl UserInfo {
    // height in cm and weight in kg 
    fn bmr(male: bool, age: usize, height: usize, weight: usize) -> usize {
        0
    }

    fn file_to_data() {
        let content = fs::read_to_string(format!("{}{}", DAT_DIR, USER_INFO_NAME)).expect("unable to open and read user data file");
        let split: Vec<&str> = content.split('\n').collect();

        println!("{}", split[0]);
        println!("{}", split[1]);
    }

    fn write_current_data(&self) {

    }
}

// adds meal 
fn eat(menu: &mut Vec<Meal>) {
   &menu.push(new_meal());

   Meal::vec_to_file(menu);
}

// shows stats
fn stats(menu: &Vec<Meal>) {

}

// removes meal
fn log(menu: &mut Vec<Meal>) {
    let mut today: Vec<&Meal> = Vec::new();
    let today_date = get_date_string();
    let mut today_count: usize = 0;

    for meal in menu.iter().rev() {
        if meal.date != today_date {
            break;
        }

        today_count += 1;
        today.push(meal);
    }

    println!("today's meals: today count: {}", today_count);
    for (i, meal) in today.iter().enumerate() {
        println!("{}: {}", i + 1, meal.calories);
    }

    let mut index: usize;
    loop {
        index = prompt_and_parse("\nindex to remove:") - 1;
        if index < today.len() {
            break;
        }
    }

    Meal::remove_index((menu.len() - today_count) + (today_count - index), menu);
    println!("meal {} removed", index + 1);
}

//helper functions
fn prompt(prompt: &str) -> String {
    let mut s = String::new();
    if prompt.len() != 0 {
        println!("{}", prompt);
    }
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().to_string()
}

fn prompt_and_parse(prompt: &str) -> usize {
    return loop {
        let mut s = String::new();
        println!("{}", prompt);
        std::io::stdin().read_line(&mut s).unwrap();

        let test = &s.trim_end().parse::<usize>();
        match test {
            Ok(ok) => break *ok,
            _ => continue,
        }
    }
}

fn get_date_string() -> String {
    let date: String = chrono::offset::Local::now().to_string();
    let now: Vec<&str> = date.split(' ').collect();

    now.get(0).expect("unable to get date").to_string()
}

fn todays_calories(menu: &Vec<Meal>) -> usize {
    let mut tally: usize = 0;
    let today = get_date_string();
    for meal in menu.iter().rev() {
        if meal.date == today {
            tally += meal.calories;
        }
        else {
            break;
        }
    }

    tally
}
