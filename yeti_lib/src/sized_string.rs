//TODO make a string able to have copy trait

use std::collections::VecDeque;

#[derive(Debug,Clone,Copy)]
pub struct SizedString<const COUNT: usize>([char;COUNT]);


impl<const COUNT: usize> TryFrom<&str> for SizedString<COUNT> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > COUNT{
            return Err("invalid length")
        }
        let mut value:VecDeque<char> = value.chars().collect();
        let mut a = [' ';COUNT];
        for ch in &mut a{
            match value.pop_front(){
                Some(n_ch) => {
                    *ch =  n_ch;
                },
                None => {
                    break;
                }
            }
            

        }
        Ok(Self{
            0:a
        })
    }
}
impl<const COUNT: usize> Into<String> for SizedString<COUNT> {

    fn into(self) -> String{
        let a = self.0.iter().collect::<String>();
        return a.trim_end().to_string();

    }
}