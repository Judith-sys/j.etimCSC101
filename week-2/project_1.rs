fn main (){
  let p : f64 = 520_000_000.00;
  let t: f64 = 5.00;
  let r: f64 = 10.0;

  let a =  p * (1.0 + (r / 100.0)).powf (t);
  let ci = a - p ;

  println!(" Compound interest is {}", ci);
}