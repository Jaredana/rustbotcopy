use std::fmt::Write;
use serenity::model::permissions;
use serenity::model::id::RoleId;
use crate::commands::*;
command!(mute(_ctx, msg, args) {
    let cache = Cache::new();
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
                        if (val.name == "Muted"){
                            //set Muted RoleId to muted_id
                            let muted_id = val.id;
                            //Check if the user already has this role since it was found in the guild
                            let is_user_muted = mem.user.read().has_role(guild_id, muted_id);
                            if(is_user_muted){
                                let user_to_mute = args.single::<String>()?;
                                //They have the role so remove it
                                mem.remove_role(muted_id);
                                msg.channel_id.say(user_to_mute + " was Un-Muted :)");
                            }
                            else if(!is_user_muted){
                                let user_to_mute = args.single::<String>()?;
                                //they don't have the role so add it to them
                                mem.add_role(muted_id);
                                msg.channel_id.say(user_to_mute + " was Muted :)");
                            }
                            found = true;
                            break;
                        }
                    }
                    //otherwise create the role
                    if(found == false){
                        msg.channel_id.say("This server doesn't have the Muted role.");
                        //.permissions(Permissions::READ_MESSAGES) needs to also include Permissions::CONNECT somehow, and modifying bit field is frowned upon..
                        //This method is from the EditRole struct, so if we could get the right set of permissions before this line, we could avoid having to use
                        //.toggle(), .set(), or .insert()
                        let mut muted_role = guild.read().id.create_role(|r|  r.hoist(true).name("Muted").permissions(Permissions::CONNECT | Permissions::READ_MESSAGES | Permissions::CHANGE_NICKNAME));
                        match muted_role{
                            Ok(mut mute_role) => {
                                msg.channel_id.say("Muted Role Created");
                                found = true;
                                //toggle, set, nor insert work here. idk wtf is going on. can't modify bits cause its private. the only thing that works is the .permissions() call above
                                //but you can only assign it 1 permission.
                                let user_to_mute = args.single::<String>()?;
                                mem.add_role(mute_role.id);
                                msg.channel_id.say(user_to_mute + " was Muted :)");
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