fn part1(){
    let input = include_str!("day2.txt");
    let mut myscore = 0;
    let mut enemy =0;
    // A / X = rock     1
    // B / Y = Paper    2 
    // C / Z = Scissors 3
    // win +6
    // Draw +3
    for row in input.lines(){
	let rowch = row.chars().collect::<Vec<char>>();
	match (rowch[0], rowch[2]) {
	    ('A','X') => {
		myscore += 4;
		enemy += 4;
	    },
	    ('A','Y') => {myscore+=8;
			    enemy+=1;},
	    ('A','Z') => {myscore+=3;
			    enemy+=7;},
	    ('B','X') => {myscore+=1;
			    enemy+=8;},
	    ('B','Y') => {myscore+=5;
			    enemy+=5;},
	    ('B','Z') => {myscore+=9;
			    enemy+=2;},
	    ('C','X') => {myscore+=7;
			    enemy+=3;},
	    ('C','Y') => {myscore+=2;
			    enemy+=9;},
	    ('C','Z') => {myscore+=6;
			  enemy+=6;},
	    (_,_) => {},
	}
    }
    println!("my score {}", myscore);
}

fn part2(){
    let input = include_str!("day2.txt");
    let mut myscore = 0;
    let mut enemy =0;
    // A / X = rock     1
    // B / Y = Paper    2 
    // C / Z = Scissors 3
    // win +6
    // Draw +3
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
    part2();
    
}
