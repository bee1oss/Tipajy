trait GetInfo{
    fn get_info(&self)->String;
}

struct Massage{
    author:String,
    text:String
}
struct Post{
    author:String,
    contents:String,
    likes:i32
}
impl GetInfo for Massage{
    fn get_info(&self) -> String {
        format!("Message author is {}\nMessage text:{}",self.author,self.text)
    }
}

impl GetInfo for Post{
    fn get_info(&self) -> String {
        format!("Post author:{}\nPost content:{}\nPost got {} likes!",self.author,self.contents,self.likes)
    }
}

fn main() {
    println!("Hello, world!");
}
