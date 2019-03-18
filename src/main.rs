#[macro_use]
extern crate clap;

mod git;
mod mob;

use mob::*;

fn main() {
    let matches = clap_app!(mob =>
        (version: "1.0.0")
        (author: "Ian Gilham")
        (about: "Mob programming handover tool")
        (@subcommand reset =>
            (about: "Deletes branch `mob-session` and `origin/mob-session`")
            (version: "1.0.0")
            (author: "Ian Gilham")
        )
        (@subcommand start =>
            (about: "Creates branch `mob-session` and pulls from `origin/mob-session`")
            (version: "1.0.0")
            (author: "Ian Gilham")
        )
        (@subcommand next =>
            (about: "Pushes all changes to `origin/mob-session` in a WIP commit")
            (version: "1.0.0")
            (author: "Ian Gilham")
        )
        (@subcommand status =>
            (about: "Display the mob session status and WIP commits")
            (version: "1.0.0")
            (author: "Ian Gilham")
        )
        (@subcommand done =>
            (about: "Squashes all changes in `mob-session` into staging of `master` and removes `mob-session` and `origin/mob-session`")
            (version: "1.0.0")
            (author: "Ian Gilham")
        )
    ).get_matches();

    let result = match matches.subcommand() {
        ("reset", Some(sub_match)) => {
            reset(sub_match)
        },
        ("start", Some(sub_match)) => {
            match start(sub_match) {
                Ok(_) => status(sub_match),
                Err(err) => Err(err),
            }

        },
        ("next", Some(sub_match)) => {
            next(sub_match)
        },
        ("done", Some(sub_match)) =>  {
            done(sub_match)
        },
        ("status", Some(sub_match)) =>  {
            status(sub_match)
        },
        ("help", _) => {
            println!("{}", matches.usage());
            Ok(())
        },
        (&_, _) => {
            println!("{}", matches.usage());
            Ok(())
        },
    };

    match result {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
        },
    }
}
