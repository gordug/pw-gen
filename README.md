# pw-gen - Password Generator
Password Generator Rust Library

Required password length is passed into new, while the special, numerical, lower and upper case calls have an optional required setting, assuring the generated password contains at least 1 character from each set.
## Example
	fn main() {
	    let password = Generator::new(10)  
			.with_special(Some(true))  
			.with_numbers(Some(true))  
			.with_lowercase(Some(true))  
			.with_uppercase(Some(true))  
			.without_similar()  
			.without_ambiguous()  
			.generate();
		println!("{}",password;
    }


## Known Issue
Current version (0.1.0) has no checks on length so requiring all 4 and setting too short a password will cause the method to loop indefinitely.