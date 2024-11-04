fn main() {
  let t: f64 = 450_000.00;
  let m: f64 = 1_500_000.00;
  let h: f64 = 750_000.00; 
  let d: f64 = 2_500_000.00;
  let a: f64 =250_000.00;

  let sum = t* 2.0 + m + h + 3.0 + d + 3.0 + a ;
  let average = sum/10.0;
  println!("The total sum is {}", sum);
  println!("The average total {} ", average);
}