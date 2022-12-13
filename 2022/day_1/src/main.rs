fn main() {
    
    // let mut running_total = 0;
    // let mut max = 0;

    let mut total_per_elf : Vec<u32> = Vec::new();
    let mut running_total = 0;

    println!("Please start inputting calorific inventory, use a blank line to signify you are done.");

    let mut skipped_last = false;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            if skipped_last == true {
                break;
            }

            total_per_elf.push(running_total);
            running_total = 0;
            skipped_last = true;

            continue;
        }
        
        if skipped_last == true {
            skipped_last = false;
        }

        let value = line.trim().parse::<u32>().unwrap();
        running_total += value;
    }

    total_per_elf.sort();

    let mut sum = 0;
    for n in 1..=3 {
        let value = total_per_elf.pop().unwrap();
        sum += value;

        println!("At position {} is, {} calories.", n, value);
    }
    println!("The grand total is, {}!", sum);

    println!("Thanks!");
}
