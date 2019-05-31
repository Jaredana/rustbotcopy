use std::fmt::Write;
use std::thread;
use std::time::Duration;
use crate::commands::*;
command!(mute(_ctx, msg, args) {
    let user_id = &msg.mentions[0].id; //Get user mentioned in message
    let my_guild = &msg.guild(); //Looks up guild in cache if its present
    match my_guild {
        Some(guild) => {
            let mut member = guild.read().member(user_id); //get the member within the guild
            let guild_roles = &guild.read().roles;
            let mut found = false;
            let guild_id = guild.read().id;
            match member {
                Ok(mut mem) => {
                    
                    //find Muted RoleID in all guild roles
                    for val in guild_roles.values() {
                        if val.name == "Muted"{
                            //set Muted RoleId to muted_id
                            let muted_id = val.id;
                            //Check if the user already has this role since it was found in the guild
                            let is_user_muted = mem.user.read().has_role(guild_id, muted_id);
                            if is_user_muted{
                                let user_to_mute = args.single::<String>()?;
                                //They have the role so remove it
                                if let Err(why) = mem.remove_role(muted_id){
                                    println!("There was en err removing role {}" ,why);
                                }
                                if let Err(why) = msg.channel_id.say(user_to_mute + " was Un-Muted :)"){
                                    println!("There was en err sending message {}" ,why);
                                }
                            }
                            else if !is_user_muted{
                                let user_to_mute = args.single::<String>()?;
                                //they don't have the role so add it to them
                                if let Err(why) = mem.add_role(muted_id) {
                                    println!("There was en err adding role {}" ,why);
                                }
                                if let Err(why) = msg.channel_id.say(user_to_mute + " was Muted :)"){
                                    println!("There was en err sending message {}" ,why);
                                }
                            }
                            found = true;
                            break;
                        }
                    }
                    //otherwise create the role
                    if found == false {
                        if let Err(why) = msg.channel_id.say("This server doesn't have the Muted role."){
                            println!("There was en err sending message {}" ,why);
                        }
                        //.permissions(Permissions::READ_MESSAGES) needs to also include Permissions::CONNECT somehow, and modifying bit field is frowned upon..
                        //This method is from the EditRole struct, so if we could get the right set of permissions before this line, we could avoid having to use
                        //.toggle(), .set(), or .insert()
                        let mut muted_role = guild.read().id.create_role(|r|  r.hoist(true).name("Muted").permissions(Permissions::CONNECT | Permissions::READ_MESSAGES | Permissions::CHANGE_NICKNAME));
                        match muted_role{
                            Ok(mut mute_role) => {
                                if let Err(why) = msg.channel_id.say("Muted Role Created"){
                                    println!("There was en creating muted role {}" ,why);
                                }
                                //toggle, set, nor insert work here. idk wtf is going on. can't modify bits cause its private. the only thing that works is the .permissions() call above
                                //but you can only assign it 1 permission.
                                let user_to_mute = args.single::<String>()?;
                                if let Err(why) = mem.add_role(mute_role.id){
                                    println!("There was en err adding role {}" ,why);
                                }
                                if let Err(why) = msg.channel_id.say(user_to_mute + " was Muted :)") {
                                    println!("There was en err sending message {}" ,why);
                                }
                            },
                            Err(e) => println!("Muted role not created {:?}", e),
                        }
                    }
                },
                Err(e) => println!("Member not found {:?}", e),
            }
        },
        None => println!("Couldn't find that Guild"),
    }
});
command!(commands(ctx, msg, _args) {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.lock();
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");
    
    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&contents) {
        println!("Error sending message: {:?}", why);
    }
});
command!(poll(_ctx, msg, args) {
    if args.is_empty() {
        msg.channel_id.say("Invalid poll structure").unwrap();
    }
    else {
        //PLEASE GOD SOMEBODY CLEAN UP THIS VARIABLE HELL IF POSSIBLE
        let argument = args.single::<String>()?;
        let data: Vec<&str> = argument.split("|").collect();
        let question = data[0];//set the question since its the first
        let sleep_time= data[(data.len() - 1)].parse::<u64>().unwrap();//Wait time in seconds
        let temp = &data[1..(data.len() -1)]; //vector of just answers with time and question removed
        let mut count = 1;
        let mut answers = Vec::new();//holds each answer tuple
        //Need to figure out a way to either have cmd defined emojis for options or way to make it use :one: :two: :three: by default
        //NEEDS FIX
        for x in temp {
            answers.push((format!("{} {}","Option",count), x, true));
            count += 1;
        }
        let pollmsg = msg.channel_id.send_message(|m| m
            .content(format!("{}{}{}","___***React to this message to reply. You have ", sleep_time, " Seconds ***___"))
            .embed(|e| e
                .title(question)
                .fields(answers)
                .colour((246, 111, 0)))).unwrap();
        //Now sleep for n seconds waiting for replies
        thread::sleep(Duration::from_secs(sleep_time));
        //After sleep, count reactions and display results
        
        //Have to get message from msgid from pollmsg variable
        let pollmsgupd = msg.channel_id.message(pollmsg.id).unwrap();
        let reactions = &pollmsgupd.reactions;
        let mut finalreacts = Vec::new();
        println!("{}{}{}", "There was ", reactions.len(), " reactions");

        //Push only the data we need to a new vec for easy displaying
        for react in reactions {
            finalreacts.push((react.count, &react.reaction_type, false));
        }
        //This is where formatting is going to be tough.NEEDS FIX
        msg.channel_id.send_message(|m| m
            .content("Poll Results:")
            .embed(|e| e
                .fields(finalreacts)
            )
        ).unwrap();
    }
});