command!(multiply(_ctx, msg, args) {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first * second;

    if let Err(why) = msg.channel_id.say(&res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, why);
    }
});

command!(addition(_ctx, msg, args) {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first + second;

    if let Err(why) = msg.channel_id.say(&res.to_string()) {
        println!("Error sending sum of {} and {}: {:?}", first, second, why);
    }
});