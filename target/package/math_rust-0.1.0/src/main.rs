use math_rust::list::{average, median, mode, standard_deviation, find_highest, find_lowest, is_sorted, reverse_vector, sort_vector };

fn main() {
  let x: Vec<i32> = vec![5, 7, 10, 10, 11];

  let mean = average(&x);
  let median = median(&x);
  let mode = mode(&x);

  let std_dev = standard_deviation(&x);
  let highest = find_highest(&x);
  let lowest = find_lowest(&x);
  let is_sorted = is_sorted(&x);
  let reverse = reverse_vector(&x);
  let sorted = sort_vector(&x);


  println!("Mean: {}", mean);
  println!("Median: {}", median);
  println!("Mode: {:?}", mode);
  println!("Standard deviation: {}", std_dev);
  println!("Highest: {:?}", highest);
  println!("Lowest: {:?}", lowest);
  println!("Is sorted: {:?}", is_sorted);
  println!("sorted {:?}", sorted);
  println!("reversed {:?}", reverse);
}





  
