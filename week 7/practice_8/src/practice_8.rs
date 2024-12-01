fn main() {
  let city_arr: [&str; 5] = ["Abuja", "Port Harcourt", "Maiduguri", "Kano", "Lagos"];
  println!("Array is: {:?}", city_arr);
  println!("Array size is: {}", city_arr.len());

  // Loop through the array using a range based on its length
  for index in 0..city_arr.len() {
      println!("City index {} is located in: {}", index, city_arr[index]);
  }
}
