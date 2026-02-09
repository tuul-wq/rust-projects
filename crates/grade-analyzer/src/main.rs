use std::array;

fn main() {
    let students: [i32; 12] = array::from_fn(|_| rand::random_range(60..=100));

    let mut passing: usize = 0;
    let mut excellent: usize = 0;
    let mut total_sum: i32 = 0;

    let mut max = students[0];
    let mut min = students[0];

    for (index, &score) in students.iter().enumerate() {
      println!("Student {}: {}", index + 1, score);

      if score >= 70 { passing += 1; }
      if score >= 90 { excellent += 1; }

      if score > max { max = score; }
      if score < min { min = score; }

      total_sum += score;
    }

    let average = total_sum as f32 / students.len() as f32;

    println!("Passing count {}", passing);
    println!("Excellent count {}", excellent);
    println!("Average score {:.2}", average);
    println!("Min score {}", min);
    println!("Max score {}", max);

    if average >= 90.0 { println!("Outstanding performance {}", average); }
    else if average >= 80.0 { println!("Great performance {}", average); }
    else if average >= 70.0 { println!("Good performance {}", average); }
    else { println!("Needs Improvement {}", average); }
}
