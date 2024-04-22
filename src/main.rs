use rand::Rng;
use std::io;
struct MagicEightBall{
    die:Die
}
impl MagicEightBall {
    fn query(&self,query:Query) -> &Response{
        let response_count: usize = self.die.responses.len();
        let secret_number = rand::thread_rng().gen_range(0..response_count);
        return &self.die.responses[secret_number];
    }
    fn new(responses:Vec<Response>) -> Self{
        let die:Die = Die{responses};
        MagicEightBall{die}
    }
}
struct Die{
    responses:Vec<Response>
}
struct Response{
    text:String,
    response_type:ResponseType,
}
enum ResponseType{
    Positive,
    NonCommittal,
    Negative,
}
struct Query{
    question:String,
}
fn main() {
    let ball:MagicEightBall = build_defaul_magic8ball();
    println!("Welcome to the magic 8 ball!");
    loop{
        println!("What question do you wish to ask the magic 8 ball?");
        let mut input = String::new();
        //read the line for input
        io::stdin().read_line(&mut input)
            .expect("Failed to readline");
        if input.trim() == "Quit"{
            println!("Good bye!");
            break;
        }
        let input_query: Query = Query{question:input};
        let shake = ball.query(input_query);
        match shake.response_type{
            ResponseType::Negative => println!("Unfortunately the magic 8 ball says: {}",shake.text),
            ResponseType::NonCommittal => println!("{}",shake.text),
            ResponseType::Positive => println!("Good fortunes, the magic 8 ball says: {}",shake.text),
        }
    }

}
fn build_defaul_magic8ball() -> MagicEightBall{
    let default_responses:Vec<Response> = vec![
        Response{text:String::from("Reply hazy, try again"),response_type:ResponseType::NonCommittal},
        Response{text:String::from("Ask again later"),response_type:ResponseType::NonCommittal},
        Response{text:String::from("Better not tell you now"),response_type:ResponseType::NonCommittal},
        Response{text:String::from("Cannot predict now"),response_type:ResponseType::NonCommittal},
        Response{text:String::from("Concentrate and ask again"),response_type:ResponseType::NonCommittal},
        Response{text:String::from("It is certain"),response_type:ResponseType::Positive},
        Response{text:String::from("It is decidedly so"),response_type:ResponseType::Positive},
        Response{text:String::from("Without a doubt"),response_type:ResponseType::Positive},
        Response{text:String::from("Yes definitely"),response_type:ResponseType::Positive},
        Response{text:String::from("You may rely on it"),response_type:ResponseType::Positive},
        Response{text:String::from("As I see it, yes"),response_type:ResponseType::Positive},
        Response{text:String::from("Most likely"),response_type:ResponseType::Positive},
        Response{text:String::from("Outlook goodn"),response_type:ResponseType::Positive},
        Response{text:String::from("Yes"),response_type:ResponseType::Positive},
        Response{text:String::from("Signs point to yes"),response_type:ResponseType::Positive},
        Response{text:String::from("Don't count on it"),response_type:ResponseType::Negative},
        Response{text:String::from("My reply is no"),response_type:ResponseType::Negative},
        Response{text:String::from("My sources say no"),response_type:ResponseType::Negative},
        Response{text:String::from("Outlook not so good"),response_type:ResponseType::Negative},
        Response{text:String::from("Very doubtful"),response_type:ResponseType::Negative}];
    MagicEightBall::new(default_responses)
}
//The way you do anything is the way you do everything