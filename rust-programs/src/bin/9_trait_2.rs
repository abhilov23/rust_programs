
pub struct NewsArticle{
  pub  headline: String,
  pub  author: String,
  pub  content: String,
}

impl Summary for NewsArticle { //using summary trait for NewsArticle
    //using summary trait for NewsArticle is a must
    fn summarize(&self) -> String {
        format!("{}, {}", self. headline, self.author)
    }
}

impl Summary1 for NewsArticle {
    //this function is over writing the summary's method
    fn summarize(&self) -> String {
        format!("{}, {}", self. headline, self.author)
    }
}


pub struct Tweet{
  pub  username: String,
  pub  content: String,
  pub  reply_to: bool,
  pub  retweet: bool,
}

pub trait Summary {
    //for every type that implements this trait, must have a summarize function
    fn summarize(&self) -> String; //shared method for summary
}

pub trait Summary1 {
    //Here is the default function, and will be automatically implement by itself.
    fn summarize(&self) -> String{
      String::from("Read more......") 
      //but this fn can be over written by other implementations
    }
}

pub trait Summary2 {
    //this is a summary function and other one is a default function
    fn summarizec(&self) -> String;
    //Here is the default function, and will be automatically implement by itself.
    fn summarize(&self) -> String{
      String::from("Read more......") 
      //but this fn can be over written by other implementations
    }
}


fn main(){
}