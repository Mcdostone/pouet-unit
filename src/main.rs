fn main() {
    // Example usage of the library
    let liters = 10.0;
    let gallons = pouet_jane::convert(liters, pouet_jane::Unit::Liter, pouet_jane::Unit::Gallon).unwrap();
    println!("{liters} liters = {gallons:.4} gallons");
}
