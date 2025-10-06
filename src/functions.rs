use std::io::{stdin, stdout, Read, Write};

pub fn nameinput() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input!");
    let playername = input.trim().to_uppercase();
    return playername.to_string();
}
//ughhh =w=

//multiple use
pub fn damage(health: &i32, damage: &i32, playerenergy: &i32, energytaken: &i32) -> (i32, i32) {
    if playerenergy < energytaken {
        return (0, 0);    
    }
    else {
        let attackresult = health - damage;
        let energyresult= playerenergy - energytaken;
        return (attackresult, energyresult);
    }
} //TODO: FIX ENERGY NEEDED IS WAY MORE THAN PUT IN THE ARGUMENT

pub fn rest(playername: &String, playerhealth: &i32, playerenergy: &i32) -> (i32, i32){
    let ph = playerhealth + 10;
    let pe = playerenergy + 10;
    println!("{:?} rested!", playername);
    return (ph, pe);
}

pub fn pause() {
    let mut stdin = stdin(); 
    let mut stdout = stdout();

    write!(stdout, "Press ENTER to continue").unwrap();
    stdout.flush().unwrap();
    let _temp = stdin.read(&mut [0u8]).unwrap();
}

pub fn action() -> String {
    let mut temp = String::new();
    stdin().read_line(&mut temp).expect("Fail!");
    let trim = temp.trim();

    match trim {
        "LIGHT ATTACK" => return "light".to_string(),
        "HEAVY ATTACK" => return "heavy".to_string(),
        "REST" => return "rest".to_string(),
        _ => return ("Something went wrong! turn on debug mode to see!").to_string(),
    }
}