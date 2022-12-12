fn main() {
    
    let mut running_total = 0;
    let mut max = 0;


    println!("Please start inputting calorific inventory, use a blank line to signify you are done.");

    let mut skipped_last = false;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            if skipped_last == true {
                break;
            }

            if running_total > max {
                max = running_total;
            }

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

    println!("Thanks! The current max calories is {}.", max);
}
