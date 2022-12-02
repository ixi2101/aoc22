fn part1(){
    let input = include_str!("day2.txt");
    let mut myscore = 0;
    // A / X = rock     1
    // B / Y = Paper    2 
    // C / Z = Scissors 3
    // win +6
    // Draw +3
    // X lost
    // Y draw
    // Z win
    for row in input.lines(){
	let rowch = row.chars().collect::<Vec<char>>();
	match (rowch[0], rowch[2]) {
	    ('A','X') => {myscore += 0 + 3},
	    ('A','Y') => {myscore += 3 + 1},
	    ('A','Z') => {myscore += 6 + 2},
	    ('B','X') => {myscore += 0 + 1},
	    ('B','Y') => {myscore += 3 + 2},
	    ('B','Z') => {myscore += 6 + 3},
	    ('C','X') => {myscore += 0 + 2},
	    ('C','Y') => {myscore += 3 + 3},
	    ('C','Z') => {myscore += 6 + 1},
	    (_,_) => {},
	}
    }
    println!("my score {}", myscore);
}

fn main() {
    part1();

    
}
