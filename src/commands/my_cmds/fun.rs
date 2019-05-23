extern crate rand;
use rand::Rng;

command!(dog(_ctx, msg, _args) {
    if let Err(why) = msg.channel_id.say(":dog:") {
        println!("Error sending message: {:?}", why);
    }
});

command!(cat(_ctx, msg, _args) {
    if let Err(why) = msg.channel_id.say(":cat:") {
        println!("Error sending message: {:?}", why);
    }
});

command!(bird(_ctx, msg, args) {
    let say_content = if args.is_empty() {
        ":bird: can find animals for you.".to_string()
    } else {
        format!(":bird: could not find animal named: `{}`.", args.full())
    };

    if let Err(why) = msg.channel_id.say(say_content) {
        println!("Error sending message: {:?}", why);
    }
});
command!(coinflip(_ctx, msg, _args) {
    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen_range(0, 2);
    if n1 == 0{
        if let Err(why) = msg.channel_id.say("Heads") {
            println!("There was en err sending message {}" ,why);
        }
    }
    else {
        if let Err(why) = msg.channel_id.say("Tails") {
            println!("There was en err sending message {}" ,why);
        }
    }
});

