use std::fs::Metadata;

trait GetInfo {
    fn get_info(&self) -> String;

    fn hide_info(&self) {
        println!("You cant read this info!");
    }
}

struct Message {
    author: String,
    text: String,
}

struct Post {
    author: String,
    contents: String,
    likes: i32,
}

impl GetInfo for Message {
    fn get_info(&self) -> String {
        format!("Message author is {}\nMessage text:{}", self.author, self.text)
    }
}

impl GetInfo for Post {
    fn get_info(&self) -> String {
        format!("Post author:{}\nPost content:{}\nPost got {} likes!", self.author, self.contents, self.likes)
    }
}

fn main() {
    let message = Message {
        author: "Mega".to_string(),
        text: "Hello guys , welcome my git repository".to_string(),
    };
    let post = Post {
        author: "Michel".to_string(),
        contents: "Heyyy yooo welcome my rust learning git repository".to_string(),
        likes: 4,
    };
    println!("{}", message.get_info());
    println!("{}", post.get_info());

    post.hide_info();
}
