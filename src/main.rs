//! # Level Game Design
//! ## Small crate!
//! level game design calculate the amount of mobs to be defeated to reach determined level


use std::io;

struct Mob {
    name: String,
    level: i32,
    experience: i32,
}


impl Mob {
    fn new(name: String,level: i32, experience: i32) -> Mob {
        Mob {
            name,
            level,
            experience,
        }
    }

    fn get_amount_to_reach_level(&self, level_experience: i32) -> i32 {
        let float_experience: f64 = self.experience as f64;
        let float_experience_level: f64 = level_experience as f64;

        let result = float_experience_level / float_experience;

        result as i32
    }
}

fn main() {
    let level: i32          = get_level();
    let experience_formula  = (level*10) + 10;
    let amount_of_mobs: i32 = get_amount_mobs();
    let mut mobs_collection: Vec<Mob> = vec![];

    for index in 0..amount_of_mobs {
        mobs_collection.push(get_mob());
    }

    for imob in mobs_collection {
        let imob_info = format!(
            "Mob Name:{} Mob Level:{} Amount:{} to reach Level:{}"
            ,imob.name, imob.level, imob.get_amount_to_reach_level(experience_formula), level);

        println!("{}", imob_info)    
    }

}

fn get_mob() -> Mob {
    let mut s = String::new();
    let mut s1 = String::new();
    let mut s2 = String::new();

    println!("Enter mob name: ");
    io::stdin().read_line(&mut s).expect("Went wrong!");
    println!("Enter mob level: ");
    io::stdin().read_line(&mut s1).expect("Went wrong!");
    println!("Enter mob experience: ");
    io::stdin().read_line(&mut s2).expect("Went wrong!");

    Mob::new(s, s2.trim().parse::<i32>().unwrap(), s2.trim().parse::<i32>().unwrap())
}

fn get_level() -> i32 {
    println!("Input level: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("went wrong!");
    s.trim().parse::<i32>().unwrap()
}

fn get_amount_mobs() -> i32 {
    println!("Input mobs amount: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("went wrong!");
    s.trim().parse::<i32>().unwrap()
}