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
    let mut deepfry_level = 5; // Default frying level
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
    let homepath = vec!["./mod.jpg"];
    let path = Path::new("./mod.jpg");
    //we need to check the attachment size here, if it's massive, we can't process it
    //NEED TO FIND THE UPPER LIMIT
    let start = SystemTime::now();
    let imgbytes = attachment.download().unwrap();
    //println!("Image Height: {:?} , Image Width {:?}, Image Size {:?}", img.height.unwrap(),img.width.unwrap(), img.size);
    let img = image::load_from_memory(&imgbytes);
    match img {
        Ok(image) => {
            //Image was created successfully
            println!("Image was opened successfully");
            //Apply filters/fuck the image by recursively opening/resizing it
            let filters = deepfry_levels.get(&deepfry_level).unwrap();
            //.resize(100, 100, FilterType::Nearest).blur(filters.0).adjust_contrast(filters.1).brighten(filters.2).huerotate(filters.3).unsharpen(filters.4, filters.5).resize(img_width,img_height, FilterType::Nearest)
            match image.resize(100, 100, FilterType::Nearest).blur(filters.0).adjust_contrast(filters.1).brighten(filters.2).huerotate(filters.3).unsharpen(filters.4, filters.5).resize(img_width,img_height, FilterType::Nearest).save(path) {
                    Ok(_i) => println!("Image fucked and saved"), //This takes foreevvvverrrr
                    Err(e) => println!("Couldn't save image... {}", e),
            };
            let since_the_epoch = start.elapsed()
                .expect("Time went backwards");
            //sends the mod.jpg file
            let _ = msg.channel_id.send_files(homepath, |m| {
                    m.content(format!("{} {:?} {}", "Processed in", since_the_epoch.as_millis(), "milliseconds"))
            });
        },
        Err(e) => println!("Couldn't open the image {}", e),
    }     
});

fn fill_deepfry_hash(my_table: &mut HashMap<i32,(f32, f32, i32, i32, f32, i32)>){
    let blur = 0.5;
    let adjust_contrast = 100.0;
    let brighten = -30;
    let huerotate = 30;
    let unsharpen = (0.5,1);
    my_table.insert(
        1,
        (blur, adjust_contrast -80.0 ,brighten+25 , huerotate-20, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        2,
        (blur + 0.2, adjust_contrast-60.0 ,brighten+20, huerotate-15, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        3,
        (blur + 0.3, adjust_contrast-40.0 ,brighten+15 , huerotate-10, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        4,
        (blur + 0.5, adjust_contrast-20.0 ,brighten+10 , huerotate-5, unsharpen.0, unsharpen.1)
    );
    my_table.insert(
        5,
        (blur, adjust_contrast ,brighten, huerotate, unsharpen.0, unsharpen.1) //Base Level fry
    );
    my_table.insert(
        6,
        (blur, adjust_contrast+20.0 ,brighten-3 , huerotate+5, unsharpen.0, unsharpen.1 +1)
    );
    my_table.insert(
        7,
        (blur, adjust_contrast+40.0 ,brighten-6 , huerotate+10, unsharpen.0, unsharpen.1 + 1)
    );
    my_table.insert(
        8,
        (blur, adjust_contrast+60.0 ,brighten-9 , huerotate+15, unsharpen.0, unsharpen.1 + 1)
    );
    my_table.insert(
        9,
        (blur, adjust_contrast+80.0 ,brighten-12 , huerotate+20, unsharpen.0, unsharpen.1 + 2)
    );
    my_table.insert(
        10,
        (blur, adjust_contrast+100.0 ,brighten-15 , huerotate+30, unsharpen.0, unsharpen.1 + 3)
    );
}