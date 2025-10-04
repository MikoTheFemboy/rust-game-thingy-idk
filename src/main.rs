use std::io::{stdin, stdout, Read, Write};
use clearscreen::clear;
/*use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;*/
//i wanted to use termion but didn't know how lol :P

static DEBUGMODE: bool = false; //global variable for debugging purposes

//single use =w=
fn nameinput() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input!");
    let playername = input.trim().to_uppercase();
    return playername.to_string();
}
//ughhh =w=

//multiple use
fn damage(health: &i32, damage: &i32, playerenergy: &i32, energytaken: &i32) -> (i32, i32) {
    if playerenergy < energytaken {
        return (0, 0);    
    }
    else {
        let attackresult = health - damage;
        let energyresult= playerenergy - energytaken;
        return (attackresult, energyresult);
    }
} //TODO: FIX ENERGY NEEDED IS WAY MORE THAN PUT IN THE ARGUMENT

fn rest(playername: &String, playerhealth: &i32, playerenergy: &i32) -> (i32, i32){
    let ph = playerhealth + 10;
    let pe = playerenergy + 10;
    println!("{:?} rested!", playername);
    return (ph, pe);
}

fn pause() {
    let mut stdin = stdin(); 
    let mut stdout = stdout();

    write!(stdout, "Press ENTER to continue").unwrap();
    stdout.flush().unwrap();
    let _temp = stdin.read(&mut [0u8]).unwrap();
}

fn printaction() {
    println!("LIGHT ATTACK (deals 20 damage | consumes 10 energy)");
    println!("HEAVY ATTACK (deals 50 damage | consumes 50 energy)");
    println!("REST (restores 10 health | restores 25 energy");
    println!("type your attack and press enter!");
}

fn action() -> String {
    let mut temp = String::new();
    stdin().read_line(&mut temp).expect("Fail!");
    let trim = temp.trim();
    if trim == "LIGHT ATTACK"{
        return "light".to_string();
    }
    else if trim == "HEAVY ATTACK"{
        return "heavy".to_string();
    }
    else if trim == "REST"{
        return "rest".to_string();
    }
    else if trim == "Fail!"{
        return "The problem is here dawg".to_string();
    }
    else {
        if DEBUGMODE == true{
            return temp.to_string();
        }
        else {
            return("Something went wrong! turn on debug mode to see!").to_string();
        }
    }
}

fn main(){
    clear().expect("Something went wrong!");

    let mut p1health = 100;
    let mut p2health = 100;

    let mut p1energy = 100;
    let mut p2energy = 100;

    let mut turn = 1; //keeps track of who's turn it is to attack, switches from 1 and 2 //now that i think about it, this should probably be a bool =w=
    
    //setup p1
    println!("Player one, enter your name!");
    let p1name = nameinput();
    println!{"your name is {:?}", p1name};
    pause();
    clear().expect("Something went wrong!");

    //setup p2
    println!("Player two, enter your name!");
    let p2name = nameinput();
    println!{"your name is {:?}", p2name};
    pause();
    clear().expect("Something went wrong!");

    let mut gamestate = true; //checks if the game is still running or not

    //main game loop
    while gamestate {
        clear().expect("Could not clear screen!");
        println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
        println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy);

        println!("");
        if turn == 1 {
            println!("{:?} turn!", p1name);
            printaction();
            let actemp = action();
            if actemp == "light"{
                let (attackresult, energytaken) = damage(&p2health, &20, &p1energy, &10);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    pause();
                    turn = 2;
                }
                else {
                    p2health = attackresult;
                    p1energy = energytaken;
                    turn = 2;
                }
            }
            else if actemp == "heavy"{
                let (attackresult, energytaken) = damage(&p2health, &50, &p1energy, &50);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    pause();
                    turn = 2;
                }
                else {
                    p2health = attackresult;
                    p1energy = energytaken;
                    turn = 2;
                }
            }
            else if actemp == "rest"{
                let (healthresult, energyresult) = rest(&p1name, &p1health, &p1energy);
                p1health = healthresult;
                p1energy = energyresult;
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                pause();
                turn = 2;
            }
            else {
                println!("Something went wrong reading your action!");
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                pause();
                turn = 2;
            }
        }
        else if turn == 2 {
            println!("{:?} turn!", p2name);
            printaction();
            let actemp = action();
            if actemp == "light"{
                let (attackresult, energytaken) = damage(&p1health, &20, &p2energy, &10);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    pause();
                    turn = 1;
                }
                else {
                    p1health = attackresult;
                    p2energy = energytaken;
                    turn = 1;
                }
            }
            else if actemp == "heavy"{
                let (attackresult, energytaken) = damage(&p1health, &50, &p2energy, &50);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    pause();
                    turn = 1;
                }
                else {
                    p1health = attackresult;
                    p2energy = energytaken;
                    turn = 1;
                }
            }
            else if actemp == "rest"{
                let (healthresult, energyresult) = rest(&p2name, &p2health, &p2energy);
                p2health = healthresult;
                p2energy = energyresult;
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                pause();
                turn = 1;
            }
            else {
                println!("Something went wrong reading your action!");
                if DEBUGMODE == true {
                   println!("{:?}", actemp);
                }
                pause();
                turn = 1;
            }
        }
        
        //checks if either players health is less than or is zero
        if p1health <= 0{
            clear().expect("Something went wrong!");
            println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
            println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy);
            println!("{:?} WON!", p2name);
            pause();
            gamestate = false; 
        }
        else if p2health <= 0{
            clear().expect("Something went wrong!");
            println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
            println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy);
            println!("{:?} WON!", p1name);
            pause();
            gamestate = false;
        }
    }
}