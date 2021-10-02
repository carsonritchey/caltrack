/* to-do
   calculate user's bmi
   stats
   truncate meal_log
   list option that shows what's been eaten

   --optional
   meal presets
*/

use std::{fs, io::{prelude::*, BufReader, Write}};
use chrono;

const DAT_DIR: &str = ".caltrack/";

const MEAL_LOG_NAME: &str = "meal_log.txt";
const USER_INFO_NAME: &str = "user_info.txt";

const SEPERATOR: char = ',';

fn main() {
    let user: UserInfo = welcome();

    // puts all previously eaten meals in one place
    let mut menu: Vec<Meal> = Meal::file_to_vec();

    println!("{}", ansi_term::Style::new().italic().paint("type 'help' for help!"));
    loop {
        main_menu(&mut menu, &user);
    }
}

// "main menu" options that user can choose 
enum MainOptions {
    Help,
    Eat,
    Stats,
    List,
    Exit
}
impl MainOptions {
    fn value(&self) -> &str {
        match *self {
            MainOptions::Help  => "help",
            MainOptions::Eat   => "eat",
            MainOptions::Stats => "stats",
            MainOptions::List  => "list",
            MainOptions::Exit  => "exit",
        }
    }
}

fn main_menu(menu: &mut Vec<Meal>, user: &UserInfo) {
    if todays_calories(menu) == 0 {
        println!("\n{}", ansi_term::Style::new().bold().paint("no meals eaten today"));
    } else {
        println!("\ntoday you've eaten {} calories", ansi_term::Color::Yellow.bold().paint(todays_calories(menu).to_string()));
    }
    loop {
        let s = prompt("");

        if s == MainOptions::Help.value() {
            println!("'eat'   to log a meal\n'stats' to see statistics about your eating habits\n'list'  to see your meal history\n'exit'  to exit");
        } else if s == MainOptions::Eat.value() {
            eat(menu);
        } else if s == MainOptions::Stats.value() {
            stats(menu, user);
        } else if s == MainOptions::List.value() {
            list(menu);
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
    name: String,
}
impl Meal {
    // returns a vector of meals taken from the meal log file
    fn file_to_vec() -> Vec<Meal> {
        let mut meals: Vec<Meal> = Vec::new();

        let f = fs::File::open(format!("{}{}", dat_path(), MEAL_LOG_NAME)).expect("unable to find meal log file");
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let split: Vec<String> = line.unwrap().to_string().split(SEPERATOR).map(|s| s.to_string()).collect();

            meals.push(Meal {calories: split[1].parse::<usize>().expect("unable to parse meal log data"), date: split.get(0).expect("no date found in meal log").to_string(), name: split.get(2).expect("no meal name found").to_string()});
        }

        return meals;
    }

    // writes meal vector to meal log file
    fn vec_to_file(menu: &Vec<Meal>) {
        // clears meal log file
        fs::OpenOptions::new().write(true).truncate(true).open(format!("{}{}", dat_path(), MEAL_LOG_NAME)).expect("unable to clear meal log file");

        // writes all meals to file
        for meal in menu {
            Meal::write_meal(meal);
        }
    }

    // appends meal to meal log 
    fn write_meal(&self) {
        let mut f = fs::OpenOptions::new().write(true).append(true).open(format!("{}{}", dat_path(), MEAL_LOG_NAME)).expect("unable to open meal log file");

        f.write_all(format!("{}{}{}{}{}\n", self.date, SEPERATOR, self.calories, SEPERATOR, self.name).as_bytes()).expect("unable to write to meal log file");
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
        name: prompt("meal name? (press enter for no name)"),
    }
}

// assumes day by day basis
struct UserInfo {
   bmr: usize, // basal metabolic rate
   goal: usize, // desired max calorie intake
}
impl UserInfo {
    // prompts user for info needed for bmr and returns it
    fn get_bmr() -> usize{
        

        UserInfo::bmr(false, 0, 0, 0)
    }

    // height in cm and weight in kg 
    fn bmr(male: bool, age: usize, height: usize, weight: usize) -> usize {
        0
    }

    fn file_to_data() -> UserInfo {
        let content = fs::read_to_string(format!("{}{}", dat_path(), USER_INFO_NAME)).expect("unable to open and read user data file");
        let split: Vec<&str> = content.split('\n').collect();

        UserInfo { bmr: split[0].parse::<usize>().unwrap(), goal: split[1].parse::<usize>().unwrap() }
    }

    fn write_current_data(&self) {
        let mut f = fs::OpenOptions::new().write(true).truncate(true).open(format!("{}{}", dat_path(), USER_INFO_NAME)).expect("unable to clear meal log file");

        f.write_all(format!("{}\n{}", self.bmr, self.goal).as_bytes()).expect("unable to write to user data file");
    }
}

// adds meal 
fn eat(menu: &mut Vec<Meal>) {
   &menu.push(new_meal());

   Meal::vec_to_file(menu);
}

// shows stats
fn stats(menu: &Vec<Meal>, user: &UserInfo) {
}

fn list(menu: &mut Vec<Meal>) {
    remove_from_today(menu);
}

// removes meal that was eaten today
fn remove_from_today(menu: &mut Vec<Meal>) {
    let mut today: Vec<&Meal> = Vec::new();
    let today_date = get_date_string();
    let mut today_count: usize = 0;

    let mut longest: usize = 0;
    for meal in menu.iter().rev() {
        if meal.date != today_date {
            break;
        }

        if digit_count(meal.calories) > longest {
            longest = digit_count(meal.calories);
        }

        today_count += 1;
        today.push(meal);
    }

    if today.len() == 0 {
        println!("no meals eaten today; nothing to remove");
        return;
    }

    println!("today's meals:");
    for (i, meal) in today.iter().enumerate() {
            let mut space = String::new();
            for _n in 0..(longest - digit_count(meal.calories)) {
                space += " "; 
            }

        print!("{}: {}{} cal", i + 1, meal.calories, space);
        if !meal.name.is_empty() {
            print!(" ({})", meal.name);
        }
        println!();
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

// initializes program
fn welcome() -> UserInfo {
    fs::create_dir_all(dat_path()).expect("unable to create dat dir");

    fs::OpenOptions::new().create(true).write(true).open(format!("{}{}", dat_path(), MEAL_LOG_NAME)).expect("unable to create meal log file");
    let mut f = fs::OpenOptions::new().create(true).write(true).read(true).open(format!("{}{}", dat_path(), USER_INFO_NAME)).expect("unable to create meal log file");

    let mut s = String::new();
    f.read_to_string(&mut s).expect("unable to read user data file");

    if s.len() == 0 {
        let user = UserInfo {bmr: UserInfo::get_bmr(), goal: prompt_and_parse("what is your daily calorie goal?")};
        user.write_current_data();

        return user;
    } else {
        return UserInfo::file_to_data();
    }
}

// helper functions
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

// returns absolute path to the dat dir
fn dat_path() -> String {
    match home::home_dir() {
        Some(p) => return format!("{}/{}", p.display().to_string(), DAT_DIR),
        None => panic!("unable to locate home directory"),
    }
} 

// returns the number of digits in an integer 
fn digit_count(n: usize) -> usize {
    if n < 10 {
        return 1;
    }

    return 1 + digit_count(n / 10);
}
