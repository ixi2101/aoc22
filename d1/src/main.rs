fn part1(){
    let input = include_str!("day1.txt");
    let mut elf_total = 0;
    let mut highest: Vec<i32> = Vec::new(); 
    for row in input.lines(){
	let row_num = row.parse::<i32>();
	match row_num{
	    Ok(x) => {elf_total += x},
	    Err(_) => {
		highest.push(elf_total);
		elf_total = 0;
	    },
	}
    }
    highest.sort_by(|a,b| b.cmp(a));
    let total = highest[0] + highest[1] + highest[2];
    println!("{:?}", highest);
    println!("Total: {}", total);
}

fn main() {
    part1();

    
}
