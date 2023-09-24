use std::str::FromStr;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Read, self};

#[derive(Clone, Debug)]
enum Command {
    Go(PathBuf)
}
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_split = s.split_once(" ").unwrap();
        if s_split.0 == "go" {
            let pb = PathBuf::from_str(s_split.1).unwrap();
            if pb.exists() {
                return Ok(Self::Go(pb));
            }
        }

        Err(())
    }

}

impl Command {
    fn exc(self) {
        match self {
            Self::Go(pat) => {
                Sequencer::from_file(pat).execute();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Choice {
    choice: String,
    command: Command
}
impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd, choice)) = s.split_once(">") {
            Ok(
                Choice { choice: choice.to_owned(), command: Command::from_str(cmd).unwrap() }
            )
        }
        else {
            Err(())
        }
    }
}

#[derive(Clone)]
enum Sequence {
    Text(String),
    Narrator(String),
    Choice(String, Vec<Choice>)
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(":") {
            if let Option::Some((text, choices)) = s.split_once(":!") {
                let mut choices_list = vec![];
                choices.split(":!:").for_each(|c| choices_list.push(Choice::from_str(c.strip_prefix("<").unwrap()).unwrap()));
                
                Ok(Self::Choice(text[1..].to_owned(), choices_list))
            }
            else {
                Err(())
            }
        }
        else if s.contains(": ") {
            if let Option::Some(splet_s) = s.split_once(": ") {
                return Ok(Self::Text(format!("[{}]: {}", splet_s.0, splet_s.1)));
            }
            else {
                return Err(());
            }

        }
        else {
            return Ok(Self::Narrator(s.to_owned()));
        }
    }
}
impl Sequence {
    fn exec_text(text: &str) {
        println!("{text}");
    }
    fn exec_narrator(text: &str) {
        println!(":::Narrator: {text}");
    }

    fn exec_choice(question: &str, choices: Vec<Choice>) {
        println!("{}", question);
        for i in 0..choices.len() {
            println!("[{}]: {}", i+1, choices[i].choice);
        }
        let mut uin = String::new();
        let _ =io::stdin().read_line(&mut uin);

        let choice: usize = uin.strip_suffix("\n").unwrap().parse::<usize>().unwrap()-1;

        let cmd = choices[choice].clone().command;
        cmd.exc();
    }
}
impl Sequence {
    pub fn exec(self) {
        match self {
            Self::Choice(question, choices) => {
                Sequence::exec_choice(&question, choices)
            }
            Self::Narrator(says) => {
                Sequence::exec_narrator(&says)
            }
            Self::Text(says) => {
               Sequence::exec_text(&says);
            }
        }
    }

}


#[derive(Clone)]
struct Sequencer {
    sequences: Vec<Sequence>
}
impl Sequencer {
    fn from_file(pat: PathBuf) -> Sequencer {
        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(pat)
            .unwrap();
        let mut file_content = String::new();
        let _ = file.read_to_string(&mut file_content).unwrap();

        let mut file_lines = Vec::new();
        file_content.split("\n").for_each(|s| {if s.len() > 0 {file_lines.push(s.to_owned())}});

        let mut sequences = Vec::new();
        file_lines.iter().for_each(|l| sequences.push(Sequence::from_str(l).unwrap()));

        Sequencer { sequences: sequences }
    }

    pub fn execute(self) {
        self.sequences.iter().for_each(move |s| {
            s.clone().exec();

            let mut buf: [u8; 1] = [0];
            let _ = io::stdin().read_exact(&mut buf);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let seq = Sequencer::from_file(
            PathBuf::from_str("/home/ig4er/end-of-time/example_game_structure/init/main.txt").unwrap()
        );
        seq.execute();
    }
}
