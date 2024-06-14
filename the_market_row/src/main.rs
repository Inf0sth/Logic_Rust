use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut client: u8 = 1;
    let mut round: u8 = 0;
    while round <= 100{
        row(client);
        client+=1;
        row(client);
        client+=1;
        row(client);
        client+=1;
        round+=1;
    }
}

fn row(client: u8){
    let mut rng: ThreadRng = rand::thread_rng();
    let mut clients_row: Vec<u8> = Vec::new();
    let decision: u8;
    decision =  rng.gen_range(1..=3);
    if decision == 1{
        clients_row.push(client);
    } else if decision == 2{
        clients_row.pop();
    } else if decision == 3{
        clients_row.remove(0);
    }
    if clients_row.len() < 1{
        println!("End");
    }
    println!("$| {:?} |IN", clients_row);
}
