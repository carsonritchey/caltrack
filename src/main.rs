use std::{fs, io::{prelude::*, BufRead, Write}, path::Path};

const DAT_DIR: &str = "dat/";
const DAILY_NAME: &str = "daily.txt";
const INFO_NAME: &str = "info.txt";
const DAILY_DAT_SEPERATOR: char = ',';

fn main() {
    println!("your bmr is {}", welcome().bmr);
}

struct Daily {

}

// assumes day by day basis
struct Info {
   bmr: usize, // basal metabolic rate
   goal: usize, // desired max calorie intake

}
impl Info {
    // height in cm and weight in kg 
    fn bmr(male: bool, age: usize, height: usize, weight: usize) -> usize {
        0
    }

    fn write_current_data() {

    }
}

fn welcome() -> Info {
    Info {
        bmr: Info::bmr(true, 0, 0, 0),
        goal: 0
    }
}

