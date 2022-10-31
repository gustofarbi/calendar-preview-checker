use clap::{Arg, ArgAction, Command};

pub fn get() -> Command {
    Command::new("check")
        .subcommand(get_preview_command())
        .subcommand(get_overlay_subcommand())
}

fn get_preview_command() -> Command {
    Command::new("preview")
        .arg(
            Arg::new("mounting-type")
                .long("mounting-type")
                .short('m')
                .action(ArgAction::Set)
                .required(true)
                .help("mounting type of calendar"),
        )
        .arg(
            Arg::new("input-file")
                .long("input-file")
                .short('i')
                .action(ArgAction::Set)
                .default_value("input.txt")
                .help("path to file with calendar ids to be checked"),
        )
        .arg(
            Arg::new("refinement")
                .long("refinement")
                .short('r')
                .action(ArgAction::SetTrue)
                .help("requests image with _ref suffix")
        )
        .arg(
            Arg::new("num-workers")
                .long("num-workers")
                .short('n')
                .action(ArgAction::Set)
                .default_value("10")
                .help("number of workers running in parallel, the number of parallel requests will be squared, DO NOT set this too high, it can flood your tcp/tls handshake pool"),
        )
}

fn get_overlay_subcommand() -> Command {
    Command::new("overlay")
        .arg(
            Arg::new("year")
                .long("year")
                .short('y')
                .action(ArgAction::Set)
                .required(true)
                .default_value("2023")
                .help("year to be checked"),
        )
        .arg(
            Arg::new("input-file")
                .long("input-file")
                .short('i')
                .action(ArgAction::Set)
                .default_value("designs.json")
                .help("path to file with calendar ids, paths and hashes to be checked (see designs.json)"),
        )
        .arg(
            Arg::new("refinement")
                .long("refinement")
                .short('r')
                .action(ArgAction::SetTrue)
                .help("requests images with _refBase and _refMask suffix")
        )
        .arg(
            Arg::new("num-workers")
                .long("num-workers")
                .short('n')
                .action(ArgAction::Set)
                .default_value("10")
                .help("number of workers running in parallel, the number of parallel requests will be squared, DO NOT set this too high, it can flood your tcp/tls handshake pool"),
        )
}
