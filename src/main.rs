fn convert_to_fahrenheit(temp: i32) {
    let result = (temp * 9/5) + 32;
    println!("A temperatura de {temp}Cº em Fahrenheit é {result}Fº: ")
}

fn convert_to_celsius(temp: i32) {
    let result = (temp - 32 ) * 5/9;
    println!("A temperatura de {temp}Fº em Celsius é {result}Cº: ")
}

fn main() {
    println!("Temperature Converter");
    convert_to_fahrenheit(30);
    convert_to_celsius(70);
}