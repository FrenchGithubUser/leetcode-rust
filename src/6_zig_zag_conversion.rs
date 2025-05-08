pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        s
    } else {
        let mut floors = vec![String::from(""); num_rows as usize];
        let mut rising = false;
        let mut floor: i32 = num_rows - 1;
        for char in s.chars() {
            floors[floor as usize].push(char);
            if floor == num_rows - 1 {
                rising = false;
            } else if floor == 0 {
                rising = true;
            }
            if rising {
                floor += 1;
            } else {
                floor -= 1;
            }
        }
        floors.reverse();
        floors.concat()
    }
}
