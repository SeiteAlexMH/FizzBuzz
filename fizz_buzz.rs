fn main() {
	let mut phrase = String::new(); 
	let array = [(3,"Fizz"),(5,"Buzz")];
	for number in 1..101{
		for element in array.iter(){
			let (divider,word) = element;
			if number % divider == 0{
				phrase += word;
			}
		}
		println!("{} : {}",number, phrase);
		phrase =String::new();
	}
}
