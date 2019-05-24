extern crate rand;
extern crate image;
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
command!(deepfry(_ctx, msg, _args) {
    let attachment = &msg.attachments[0]; //This is the actual Attachment object
    //let img_height = attachment.height.unwrap() as u32;
    //let img_width = attachment.width.unwrap() as u32;
    let imgbytes = attachment.download(); //This is the bytes of the attachment
    match imgbytes {
        Ok(bytes_v) => {
            //println!("Image Height: {:?} , Image Width {:?}, Image Size {:?}", img.height.unwrap(),img.width.unwrap(), img.size);
            let img = image::load_from_memory(&bytes_v);
            match img {
                Ok(image) => {
                    //Image was created successfully
                    println!("Image was opened successfully");
                    //Apply filters/fuck the image by recursively opening/resizing it
                    image.brighten(32);
                    
                    /*This line is going to take some work. Need to convert image to either a File or need to store it locally to
                    give this method a Path to the image.
                    msg.channel_id.send_files(image); */
                },
                Err(e) => println!("Couldn't open the image {}", e),
            }
        },
        Err(e) => println!("Couldn't download the attachment {}", e),
    }
});

