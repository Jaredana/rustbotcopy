extern crate rand;
extern crate image;
use std::path::Path;
use std::collections::HashMap;
use image::*;
use rand::Rng;
use std::time::SystemTime;
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
command!(deepfry(_ctx, msg, args) {
    let mut deepfry_level = 3; // Default frying level
    if args.is_empty() {
        println!("Frying level default");
    }
    else{
        deepfry_level = args.single::<i32>()?;
    }
    let mut deepfry_levels: HashMap<i32, (f32, f32, i32, i32, f32, i32)> = HashMap::new();
    fill_deepfry_hash(&mut deepfry_levels);
    let attachment = &msg.attachments[0]; //This is the actual Attachment object
    let img_height = attachment.height.unwrap() as u32;
    let img_width = attachment.width.unwrap() as u32;
    let homepath = vec!["./test.jpg"];
    let path = Path::new("./test.jpg");
    //we need to check the attachment size here, if it's massive, we can't process it
    //NEED TO FIND THE UPPER LIMIT
    let start = SystemTime::now();
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
                    let filters = deepfry_levels.get(&deepfry_level).unwrap();
                    //
                    match image.resize(100, 100, FilterType::Gaussian).blur(filters.0).adjust_contrast(filters.1).brighten(filters.2).huerotate(filters.3).unsharpen(filters.4, filters.5).resize(img_width,img_height, FilterType::Gaussian).save(path) {
                        Ok(_i) => println!("Image fucked and saved"), //This takes foreevvvverrrr
                        Err(e) => println!("Couldn't save image... {}", e),
                    };
                    let since_the_epoch = start.elapsed()
                        .expect("Time went backwards");
                    //sends the test.jpg file
                    let _ = msg.channel_id.send_files(homepath, |m| {
                        m.content(format!("{} {:?} {}", "Processed in", since_the_epoch.as_millis(), "milliseconds"))
                    });
                },
                Err(e) => println!("Couldn't open the image {}", e),
            }
        },
        Err(e) => println!("Couldn't download the attachment {}", e),
    }
});

fn fill_deepfry_hash(my_table: &mut HashMap<i32,(f32, f32, i32, i32, f32, i32)>){
    let blur = 0.1;
    let adjust_contrast = 0.0;
    let brighten = -1;
    let huerotate = 1;
    let unsharpen = (0.7,1);
    my_table.insert(
        1,
        (blur, adjust_contrast ,brighten , huerotate, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        2,
        (blur + 0.2, adjust_contrast+30.0 ,brighten-1 , huerotate+6, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        3,
        (blur + 0.3, adjust_contrast+40.0 ,brighten-2 , huerotate+8, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        4,
        (blur + 0.5, adjust_contrast+50.0 ,brighten-3 , huerotate+10, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        5,
        (blur + 0.75, adjust_contrast+60.0 ,brighten-4 , huerotate+12, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        6,
        (blur + 1.0, adjust_contrast+70.0 ,brighten-5 , huerotate+16, unsharpen.0, unsharpen.1 +1)
    );
    my_table.insert(
        7,
        (blur + 1.25, adjust_contrast+80.0 ,brighten-6 , huerotate+20, unsharpen.0, unsharpen.1 + 1)
    );
    my_table.insert(
        8,
        (blur + 1.5, adjust_contrast+100.0 ,brighten-7 , huerotate+24, unsharpen.0, unsharpen.1 + 1)
    );
    my_table.insert(
        9,
        (blur+ 1.75, adjust_contrast+120.0 ,brighten-8 , huerotate+28, unsharpen.0, unsharpen.1 + 2)
    );
    my_table.insert(
        10,
        (blur+ 2.0, adjust_contrast+140.0 ,brighten-10 , huerotate+16, unsharpen.0, unsharpen.1 + 3)
    );
}