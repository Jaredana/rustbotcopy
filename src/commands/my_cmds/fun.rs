extern crate rand;
extern crate image;
use std::path::Path;
use image::*;
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
    let img_height = attachment.height.unwrap() as u32;
    let img_width = attachment.width.unwrap() as u32;
    let homepath = vec!["./test.jpg"];
    let path = Path::new("./test.jpg");
    //we need to check the attachment size here, if it's massive, we can't process it
    //NEED TO FIND THE UPPER LIMIT
    let imgbytes = attachment.download(); //This is the bytes of the attachment
    match imgbytes {
        Ok(bytes_v) => {
            //println!("Image Height: {:?} , Image Width {:?}, Image Size {:?}", img.height.unwrap(),img.width.unwrap(), img.size);
            let img = image::load_from_memory_with_format(&bytes_v, ImageFormat::JPEG);
            match img {
                Ok(image) => {
                    //Image was created successfully
                    println!("Image was opened successfully");
                    //Apply filters/fuck the image by recursively opening/resizing it
                    match image.resize(100, 100, FilterType::Gaussian).blur(1.5).adjust_contrast(50.0).brighten(-10).huerotate(10).unsharpen(4.3, 6).resize(img_width,img_height, FilterType::Gaussian).save(path) {
                        Ok(_i) => println!("Image fucked and saved"),
                        Err(e) => println!("Couldn't save image... {}", e),
                    };
                    //sends the test.jpg file
                    let _ = msg.channel_id.send_files(homepath, |m| {
                        m.content("")
                    });
                },
                Err(e) => println!("Couldn't open the image {}", e),
            }
        },
        Err(e) => println!("Couldn't download the attachment {}", e),
    }
});

