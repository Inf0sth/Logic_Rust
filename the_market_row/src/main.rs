/*We have to create 2 or 4 functions, in a market row, we have some posibilities like:
- Have a new person,
- Serve a person
- A customer got tired of waiting and left.
Furthermore, customers do not stay in the same position
- Clients are moving forward

Use:
- Stack
- Row
- Random

Clients will be represented by numbers, numbers will advance
If a customer leaves, or is served, 
their number is taken as past and will not be repeated in 100 rounds.
*/
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut clients_row = Vec::new();
    let mut client: u8 = 1;
    let mut decision: u8;
    let mut round: u8 = 0;
    clients_row.push(client);
    client = client+1;
    while round <= 100{
        decision =  rng.gen_range(1..=3);
        if decision == 1{
            clients_row.push(client);
        } else if decision == 2{
            clients_row.pop();
        } else if decision == 3{
            clients_row.remove(0);
        }
        if clients_row.lenght() < 1{
            println("End");
        }
        round++;
    }

    println!("Hello, world!");
}
