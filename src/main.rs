use std::io;

fn main() {

    // normal game values for unrated average
    let normaltime:f32 = 45.0;
    let normalxp:f32 = 3850.0;

    // spikerush values
    let spiketime:f32 = 12.0;
    let spikexp:f32 = 1000.0;

    // deathmatch values
    let dmtime:f32 = 10.0;
    let dmxp:f32 = 900.0;
    
    // Get the XPNEEDED
    loop { 
        println!("Input your XP needed to next unlock.");
        let mut xpneeded = String::new();

        io::stdin()
            .read_line(&mut xpneeded)
            .expect("Failed to get number.");

        let xpneeded: f32 = match xpneeded.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    // get normal game calculations
    println!("Normal game results");
    results(xpneeded, normalxp, normaltime);
    empty();

    // get spike game calculations
    println!("Spikerush game results");
    results(xpneeded, spikexp, spiketime);
    empty();

    // get dm game calculations
    println!("deathmatch game results");
    results(xpneeded, dmxp, dmtime);
    empty();

    break;

    }
}

// a: xpneeded, b: gamemodexp, c: gamemodetime
fn results (a: f32, b: f32, c: f32) {
    let games:f32 = (a / b).ceil() as f32;
    let duration: f32 = games*c as f32;
    let hour: f32 = (games * c/60.0).floor() as f32;
    let minute: f32 = duration%60.0;
    

    if duration > c {
        println!("You need to play {} games, taking you ~{}:{} hours.", games, hour, minute);
    } else {
        println!("You need to play {} games, taking you ~{} minutes.", games, c);
    }

}

fn empty () {
    println!("")
}