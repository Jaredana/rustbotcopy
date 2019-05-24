extern crate serenity;
extern crate rand;
use std::{collections::HashMap, sync::Arc};
mod my_cmds;
use serenity::{
    client::bridge::gateway::{ShardManager},
    framework::standard::{
        help_commands, Args, CommandOptions, DispatchError, HelpBehaviour, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
};

// This imports `typemap`'s `Key` as `TypeMapKey`.

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn initialize(client: &mut Client) {
    
    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Commands are equivalent to:
    // "~about"
    // "~emoji cat"
    // "~emoji dog"
    // "~multiply"
    // "~ping"
    // "~some long command"
    client.with_framework(
        // Configures the client, allowing for options to mutate how the
        // framework functions.
        //
        // Refer to the documentation for
        // `serenity::ext::framework::Configuration` for all available
        // configurations.
        StandardFramework::new()
        .configure(|c| c
            .allow_whitespace(true)
            .on_mention(true)
            .prefix("~")
            // A command that will be executed
            // if nothing but a prefix is passed.
            .prefix_only_cmd(my_cmds::other::about)
            // You can set multiple delimiters via delimiters()
            // or just one via delimiter(",")
            // If you set multiple delimiters, the order you list them
            // decides their priority (from first to last).
            //
            // In this case, if "," would be first, a message would never
            // be delimited at ", ", forcing you to trim your arguments if you
            // want to avoid whitespaces at the start of each.
            .delimiters(vec![", ", ","]))

        // Set a function to be called prior to each command execution. This
        // provides the context of the command, the message that was received,
        // and the full name of the command that will be called.
        //
        // You can not use this to determine whether a command should be
        // executed. Instead, `set_check` is provided to give you this
        // functionality.
        .before(|ctx, msg, command_name| {
            println!("Got command '{}' by user '{}'",
                     command_name,
                     msg.author.name);

            // Increment the number of times this command has been run once. If
            // the command's name does not exist in the counter, add a default
            // value of 0.
            let mut data = ctx.data.lock();
            let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");
            let entry = counter.entry(command_name.to_string()).or_insert(0);
            *entry += 1;

            true // if `before` returns false, command processing doesn't happen.
        })
        // Similar to `before`, except will be called directly _after_
        // command execution.
        .after(|_, _, command_name, error| {
            match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        // Set a function that's called whenever an attempted command-call's
        // command could not be found.
        .unrecognised_command(|_, _, unknown_command_name| {
            println!("Could not find command named '{}'", unknown_command_name);
        })
        // Set a function that's called whenever a message is not a command.
        .message_without_command(|_, message| {
            println!("Message is not a command '{}'", message.content);
        })
        // Set a function that's called whenever a command's execution didn't complete for one
        // reason or another. For example, when a user has exceeded a rate-limit or a command
        // can only be performed by the bot owner.
        .on_dispatch_error(|_ctx, msg, error| {
            if let DispatchError::RateLimited(seconds) = error {
                let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
            }
        })
        // Can't be used more than once per 5 seconds:
        .simple_bucket("fun", 5)
        // Can't be used more than 2 times per 30 seconds, with a 5 second delay:
        .bucket("complicated", 5, 30, 2)
        
        // You can use the simple `help(help_commands::with_embeds)` or
        // customise your help-menu via `customised_help()`.
        .customised_help(help_commands::with_embeds, |c| {
                // This replaces the information that a user can pass
                // a command-name as argument to gain specific information about it.
                c.individual_command_tip("Hello! こんにちは！Hola! Bonjour! 您好! Здравствуйте!\n\
                If you want more information about a specific command, just pass the command as argument.")
                // Some arguments require a `{}` in order to replace it with contextual information.
                // In this case our `{}` refers to a command's name.
                .command_not_found_text("Could not find: `{}`.")
                // Define the maximum Levenshtein-distance between a searched command-name
                // and commands. If the distance is lower than or equal the set distance,
                // it will be displayed as a suggestion.
                // Setting the distance to 0 will disable suggestions.
                .max_levenshtein_distance(3)
                // On another note, you can set up the help-menu-filter-behaviour.
                // Here are all possible settings shown on all possible options.
                // First case is if a user lacks permissions for a command, we can hide the command.
                .lacking_permissions(HelpBehaviour::Hide)
                // If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
                .lacking_role(HelpBehaviour::Nothing)
                // The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
                .wrong_channel(HelpBehaviour::Strike)
                // Serenity will automatically analyse and generate a hint/tip explaining the possible
                // cases of ~~strikethrough-commands~~, but only if
                // `striked_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
                // If the `String` is not empty, your given `String` will be used instead.
                // If you pass in a `None`, no hint will be displayed at all.
                 })
        
        // Command that will repeat passed arguments and remove user and
        // role mentions with safe alternative.
        
        .group("Mod", |g| g
            .desc("A suite of tools for holding back the hoard")
            .command("Mute", |c| c
                .desc("Mute a heathen")
                .cmd(my_cmds::modcmd::mute)
                .batch_known_as(vec!["mute"])
                //.required_permissions(Permissions::ADMINISTRATOR)
            )
            .command("commands", |c| c
            // Make this command use the "complicated" bucket.
                .bucket("complicated")
                .cmd(my_cmds::modcmd::commands))
        )
        .group("Fun", |g| g
            // Sets multiple prefixes for a group.
            // This requires us to call commands in this group
            // via `~emoji` (or `~e`) instead of just `~`.
            
            // Set a description to appear if a user wants to display a single group
            // e.g. via help using the group-name or one of its prefixes.
            .desc("A group of fun commands")
            
            // Sets a command that will be executed if only a group-prefix was passed.
            .command("cat", |c| c
                .desc("Sends an emoji with a cat.")
                .batch_known_as(vec!["kitty", "neko"]) // Adds multiple aliases
                .bucket("fun") // Make this command use the "emoji" bucket.
                .cmd(my_cmds::fun::cat)
                 // Allow only administrators to call this:
                .required_permissions(Permissions::ADMINISTRATOR))
            .command("dog", |c| c
                .desc("Sends an emoji with a dog.")
                .bucket("fun")
                .cmd(my_cmds::fun::dog))
            .command("bird", |c| c
                .desc("Sends an emoji with a bird.")
                .bucket("fun")
                .cmd(my_cmds::fun::bird))   
            .command("coinflip", |c| c
                .desc("Flips a coin")
                .bucket("fun")
                .cmd(my_cmds::fun::coinflip))
            .command("deepfry", |c| c
                .desc("Deep frys an image")
                .bucket("complicated")
                .cmd(my_cmds::fun::deepfry)
            )
        )
        .group("Math", |g| g
            // Sets a single prefix for this group.
            // So one has to call commands in this group
            // via `~math` instead of just `~`.
            .prefix("math")
            .command("multiply", |c| c
                .known_as("*") // Lets us also call `~math *` instead of just `~math multiply`.
                .cmd(my_cmds::math::multiply))
            .command("add", |c| c.known_as("+").cmd(my_cmds::math::addition))
        )
        .group("Other", |g| g
            .command("latency", |c| c
                .cmd(my_cmds::other::latency))
            .command("ping", |c| c
                .check(owner_check) // User needs to pass this test to run command
                .cmd(my_cmds::other::ping))
            .command("role", |c| c
                .cmd(my_cmds::other::about_role)
            // Limits the usage of this command to roles named:
                .allowed_roles(vec!["mods", "ultimate neko"]))
            .command("say", |c| c
                .cmd(my_cmds::other::say))
            .command("about", |c| c.cmd(my_cmds::other::about))
        )
        .group("Owner", |g| g
            // This check applies to every command on this group.
            // User needs to pass the test for the command to execute.
            .check(admin_check)
            .command("am i admin", |c| c
                .cmd(am_i_admin)
                .guild_only(true))
        ),
    );
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

// Commands can be created via the `command!` macro, to avoid manually typing
// type annotations.
//
// This may bring more features available for commands in the future. See the
// "multiply" command below for some of the power that the `command!` macro can
// bring.


// Repeats what the user passed as argument but ensures that user and role
// mentions are replaced with a safe textual alternative.
// In this example channel mentions are excluded via the `ContentSafeOptions`.


// A function which acts as a "check", to determine whether to call a command.
//
// In this case, this command checks to ensure you are the owner of the message
// in order for the command to be executed. If the check fails, the command is
// not called.
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    // Replace 7 with your ID
    msg.author.id == 7
}

// A function which acts as a "check", to determine whether to call a command.
//
// This check analyses whether a guild member permissions has
// administrator-permissions.
fn admin_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    if let Some(member) = msg.member() {

        if let Ok(permissions) = member.permissions() {
            return permissions.administrator();
        }
    }

    false
}
command!(am_i_admin(_ctx, msg, _args) {
    if let Err(why) = msg.channel_id.say("Yes you are.") {
        println!("Error sending message: {:?}", why);
    }
});
