use std::io;
fn main() {
    println!("Guess the word!");

    loop {

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //trim 
    let guess = guess.trim();

    let guess_length = guess.len();
    
    println!("the length of the word is {}", guess_length );

    if guess_length == 5 {
      
        println!("the word is {}", guess);

        get_character(guess);
         break;

         // Convert &str back to String when passing to the function
       
  
      } else {
  
          println!("please input a 5 letter word!");
  
      }

    }   

}




pub fn get_character (guess  :&str) {

   let word :&str  = guess;

   let letter_one: char =  word.chars().nth(0).unwrap(); 
   
   let letter_two: char =  word.chars().nth(1).unwrap(); 
   
   let letter_three: char =  word.chars().nth(2).unwrap(); 
   
   let letter_four: char =  word.chars().nth(3).unwrap(); 
   
   let letter_five: char =  word.chars().nth(4).unwrap(); 

   print!("the word you guessed is {}{}{}{}{}",letter_one,letter_two,letter_three,letter_four,letter_five);

   let correct_letter_one : char = 's';
   let correct_letter_two : char = 't';
   let correct_letter_three : char = 'r';
   let correct_letter_four : char = 'a';
   let correct_letter_five : char = 'y';

   if letter_one == correct_letter_one {
    print!("the letter:{} is in the correct position",letter_one);

   } 


   if letter_two == correct_letter_two {
    print!("the letter:{} is in the correct position",letter_two);

   } 


   if letter_three == correct_letter_three {
    print!("the letter:{} is in the correct position",letter_three);

   } 



   if letter_four == correct_letter_four {
    print!("the letter:{} is in the correct position",letter_four);

   } 


   if letter_five == correct_letter_five {
    print!("the letter:{} is in the correct position",letter_five);

   } 


   
   if letter_one == correct_letter_two {
    print!("the letter:{} is in the correct word",letter_one);

   }


   if letter_one == correct_letter_three {
    print!("the letter:{} is in the correct word",letter_one);

   } 


   if letter_one == correct_letter_four {
    print!("the letter:{} is in the correct word",letter_one);

   } 


   if letter_one == correct_letter_five {
    print!("the letter:{} is in the correct word",letter_one);

   } 

   if letter_two == correct_letter_one {
    print!("the letter:{} is in the correct word",letter_two);

   } 

 


   if letter_two == correct_letter_three {
    print!("the letter:{} is in the correct word",letter_two);

   } 



   if letter_two == correct_letter_four {
    print!("the letter:{} is in the correct word",letter_two);

   } 


   if letter_two == correct_letter_five {
    print!("the letter:{} is in the correct word",letter_two);

   } 



   if letter_three == correct_letter_one {
    print!("the letter:{} is in the correct word",letter_three);

   } 

   
   if correct_letter_three == correct_letter_two {
    print!("the letter:{} is in the correct word",letter_three);

   } 





   if letter_three == correct_letter_four {
    print!("the letter:{} is in the correct word",letter_three);

   } 


   if letter_three == correct_letter_five {
    print!("the letter:{} is in the correct word",letter_three);

   } 

   if letter_four == correct_letter_one {
    print!("the letter:{} is in the correct word",letter_four);

   } 

   
   if letter_four == correct_letter_two {
    print!("the letter:{} is in the correct word",letter_four);

   } 


   if letter_four == correct_letter_three {
    print!("the letter:{} is in the correct word",letter_four);

   } 



  


   if letter_four == correct_letter_five {
    print!("the letter:{} is in the correct word",letter_four);

   } 



   if letter_five == correct_letter_one {
    print!("the letter:{} is in the correct word",letter_four);

   } 

   
   if letter_five == correct_letter_two {
    print!("the letter:{} is in the correct word",letter_five);

   } 


   if letter_five == correct_letter_three {
    print!("the letter:{} is in the correct word",letter_five);

   } 



   if letter_five == correct_letter_four {
    print!("the letter:{} is in the correct word",letter_five);

   } 

  



  
}





