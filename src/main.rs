use clearscreen::clear;
mod functions;

static DEBUGMODE: bool = false; //global variable for debugging purposes

fn main(){
    clear().expect("Something went wrong!");

    let mut p1health = 100;
    let mut p2health = 100;

    let mut p1energy = 100;
    let mut p2energy = 100;

    let mut turn = 1; //keeps track of who's turn it is to attack, switches from 1 and 2 //now that i think about it, this should probably be a bool =w=
    
    //setup p1
    println!("Player one, enter your name!");
    let p1name = functions::nameinput();
    println!{"your name is {:?}", p1name};
    functions::pause();
    clear().expect("Something went wrong!");

    //setup p2
    println!("Player two, enter your name!");
    let p2name = functions::nameinput();
    println!{"your name is {:?}", p2name};
    functions::pause();
    clear().expect("Something went wrong!");

    let mut gamestate = true; //checks if the game is still running or not

    //main game loop
    while gamestate {
        clear().expect("Could not clear screen!");
        println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
        println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy); 
        println!("");
        println!("LIGHT ATTACK (deals 20 damage | consumes 10 energy)");
        println!("HEAVY ATTACK (deals 50 damage | consumes 50 energy)");
        println!("REST (restores 10 health | restores 25 energy");
        println!("type your attack and press enter!");

        println!("");
        if turn == 1 {
            println!("{:?} turn!", p1name);
            let actemp = functions::action();

            if actemp == "light"{
                let (attackresult, energytaken) = functions::damage(&p2health, &20, &p1energy, &10);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    functions::pause();
                    turn = 2;
                }
                else {
                    p2health = attackresult;
                    p1energy = energytaken;
                    turn = 2;
                }
            }
            else if actemp == "heavy"{
                let (attackresult, energytaken) = functions::damage(&p2health, &50, &p1energy, &50);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    functions::pause();
                    turn = 2;
                }
                else {
                    p2health = attackresult;
                    p1energy = energytaken;
                    turn = 2;
                }
            }
            else if actemp == "rest"{
                let (healthresult, energyresult) = functions::rest(&p1name, &p1health, &p1energy);
                p1health = healthresult;
                p1energy = energyresult;
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                functions::pause();
                turn = 2;
            }
            else {
                println!("Something went wrong reading your action!");
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                functions::pause();
                turn = 2;
            }
        }
        else if turn == 2 {
            println!("{:?} turn!", p2name);
            let actemp = functions::action();
            if actemp == "light"{
                let (attackresult, energytaken) = functions::damage(&p1health, &20, &p2energy, &10);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    functions::pause();
                    turn = 1;
                }
                else {
                    p1health = attackresult;
                    p2energy = energytaken;
                    turn = 1;
                }
            }
            else if actemp == "heavy"{
                let (attackresult, energytaken) = functions::damage(&p1health, &50, &p2energy, &50);
                if  attackresult == 0 || energytaken == 0{
                    println!("You do not have enough energy!");
                    if DEBUGMODE == true {
                        println!("{:?}", actemp);
                    }
                    functions::pause();
                    turn = 1;
                }
                else {
                    p1health = attackresult;
                    p2energy = energytaken;
                    turn = 1;
                }
            }
            else if actemp == "rest"{
                let (healthresult, energyresult) = functions::rest(&p2name, &p2health, &p2energy);
                p2health = healthresult;
                p2energy = energyresult;
                if DEBUGMODE == true {
                    println!("{:?}", actemp);
                }
                functions::pause();
                turn = 1;
            }
            else {
                println!("Something went wrong reading your action!");
                if DEBUGMODE == true {
                   println!("{:?}", actemp);
                }
                functions::pause();
                turn = 1;
            }
        }
        
        //checks if either players health is less than or is zero
        if p1health <= 0{
            clear().expect("Something went wrong!");
            println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
            println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy);
            println!("{:?} WON!", p2name);
            functions::pause();
            gamestate = false; 
        }
        else if p2health <= 0{
            clear().expect("Something went wrong!");
            println!("{:?} has {:?} health and {:?} energy", p1name, p1health, p1energy);
            println!("{:?} has {:?} health and {:?} energy", p2name, p2health, p2energy);
            println!("{:?} WON!", p1name);
            functions::pause();
            gamestate = false;
        }
    }
}