
#[warn(dead_code)]
#[allow(dead_code)]
#[allow(unused_imports)]


pub mod list {
    
  pub fn average(x: &Vec<i32>) -> f64 {
    let numm = x.len();
    let mut summ = 0;
    for num in x {
      summ += num;
    }
    let avg = (summ as f64) / (numm as f64);
    return avg;
  }
  
  pub fn median(x: &Vec<i32>) -> f64 {
    let mut arr = x.clone();
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
  
    let mid = x.len() / 2;
    if x.len() % 2 == 0 {
      return (arr[mid - 1] + x[mid]) as f64 / 2.0;
    } else {
      return arr[mid] as f64;
    };  
  }
  
  pub fn mode(x: &Vec<i32>) -> Option<i32> {
    let mut freq = std::collections::HashMap::new();
    
    for num in x {
      let count = freq.entry(num).or_insert(0);
      *count += 1;
    }
    let mut mode = None;
    let mut max_count = 0;
    
    for (number, count) in &freq {
      if *count > max_count {
        mode = Some(**number);
        max_count = *count;
      }
    }
    return mode;
  }
  
  pub fn reverse_vector(x: &Vec<i32>) -> Vec<i32> {
    let mut arr = x.clone();
    arr.reverse();
    return arr.to_vec();
  }
  
  pub fn sort_vector(x: &Vec<i32>) -> Vec<i32> {
    let mut arr = x.clone();
    arr.sort();
    return arr.to_vec();
  }
  
  pub fn find_highest(x: &Vec<i32>) -> Option<&i32> {
      if x.is_empty() {
          return None;
      }
      let mut highest = &x[0];
      for num in x {
          if num > highest {
              highest = num;
          }
      }
      return Some(highest);
  }
  pub fn find_lowest(x: &Vec<i32>) -> Option<&i32> {
      if x.is_empty() {
          return None;
      }
      let mut lowest = &x[0];
      for num in x {
          if num < lowest {
              lowest = num;
          }
      }
      return Some(lowest);
  }
  
  pub fn is_sorted(x: &Vec<i32>) -> bool {
      if x.len() <= 1 {
          return true;
      }
      for i in 1..x.len() {
          if x[i] < x[i-1] {
              return false;
          }
      }
      return true;
  }
  
  pub fn standard_deviation(x: &Vec<i32>) -> f64 {
      let n = x.len() as f64;
      let mean = (x.iter().sum::<i32>()) as f64 / n;
      let variance = (x.iter().map(|num| (num - mean as i32).pow(2)).sum::<i32>()) as f64 / n;
      variance.sqrt()
  }

  pub fn random_vector(len : i32) -> Vec<i32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut rand_vect : Vec<i32>  = Vec::new();

    for _ in 0..len{
      let random_number: i32 = rng.gen();
      rand_vect.push(random_number);
    }
    return rand_vect
  }
    
} 
  
  
  
  
  
  