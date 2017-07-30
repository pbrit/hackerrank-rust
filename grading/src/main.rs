use std::io;

fn main() {
    let mut number_of_grades = String::new();
    io::stdin().read_line(&mut number_of_grades).unwrap();
    let number_of_grades: usize = number_of_grades.trim().parse().unwrap();

    for _ in 0..number_of_grades {
        let mut grade = String::new();

        io::stdin().read_line(&mut grade).unwrap();

        let grade: u8 = grade.trim().parse().unwrap();

        if grade < 38 {
            println!("{}", grade);
        } else {
            let reminder = grade % 5;
            let normal_grade = if reminder >= 3 {
                5 * (grade / 5) + 5
            } else {
                grade
            };

            println!("{}", normal_grade);
        }
    }
}
