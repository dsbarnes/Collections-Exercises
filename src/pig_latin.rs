 pub mod mod_pig_latin{
 
     fn is_vowel(mut input: String) -> bool {
         let first = input.remove(0)
             .to_uppercase().next().unwrap();
 
         match first {
             'A' | 'E' | 'I' | 'O' | 'U' =>
                 return true,
             _ => return false,
         }
 
     }
 
     pub fn to_pig_latin(mut input: String) -> String {
         if is_vowel(input.clone()) {
             return format!("{}-hay", input);
         }
 
         let first = input.remove(0);
         format!("{}-{}ay", input, first)
     }
 }
