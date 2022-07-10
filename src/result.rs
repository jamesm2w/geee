use std::fs;
use std::env;
use json;
use crate::party::Party;

#[derive(Debug)]
pub struct MPInfo {
    pub name: String,
    pub party: Party
}

impl MPInfo {
    pub fn new(name: &str, party: &str) -> MPInfo {
        MPInfo {
            name: name.to_string(),
            party: Party::parse(party)
        }
    }
}

#[derive(Debug)]
pub struct SeatResult {
    pub onscode: String,
    pub name: String,
    pub electorate: u32,
    pub turnout: u32,
    pub winner: MPInfo,
    pub result: String,
    pub candidates: Vec<Candidate>
}

impl <'a> SeatResult {
    pub fn new(onscode: &'a str, name: &'a str, electorate: u32, turnout: u32, winner: MPInfo, result: &'a str, candidates: Vec<Candidate>) -> SeatResult {
        SeatResult {
            onscode: onscode.to_string(),
            name: name.to_string(),
            electorate,
            turnout,
            winner,
            result: result.to_string(),
            candidates
        }
    }

    pub fn add_candidate(&mut self, candidate: Candidate) {
        self.candidates.push(candidate);
    }
}

#[derive(Debug)]
pub struct Candidate {
    pub candidate: String,
    pub party: Party,
    pub votes: u32,
    pub percent_change: f32,
    pub incumbent: bool,
    pub elected: bool
}

impl Candidate {
    pub fn new(candidate: &str, party: &str, votes: u32, percent_change: f32, incumbent: bool, elected: bool) -> Candidate {
        Candidate {
            candidate: String::from(candidate),
            party: Party::parse(party),
            votes,
            percent_change,
            incumbent,
            elected
        }
    }
}

type Election = Vec<SeatResult>;

pub fn parse(file_name: &str) -> Option<Election> {
    let mut results: Election = Vec::new();

    let mut file_path = env::current_dir().unwrap();
    file_path.push(file_name);

    let file_contents = fs::read_to_string(file_path).expect("Could not read file");

    if let Ok(result_json) = json::parse(&file_contents) 
    {

        for (_, seat) in result_json.entries() {

            let winner = MPInfo::new(
                seat["mp"]["name"].as_str().unwrap(), 
                seat["mp"]["party"].as_str().unwrap()
            );

            let mut candidates: Vec<Candidate> = Vec::new();

            for candidate in seat["candidates"].members() {
                let candidate = Candidate::new(
                    candidate["candidate"].as_str().unwrap(),
                    candidate["party"].as_str().unwrap(),
                    candidate["votes"].as_u32().unwrap(),
                    candidate["change"].to_string().parse::<f32>().unwrap(),
                    candidate["incumbent"].as_bool().unwrap(),
                    candidate["elected"].as_bool().unwrap()
                );
                candidates.push(candidate);
            }

            candidates.sort_by(|a, b| b.votes.cmp(&a.votes));

            let seat_result = SeatResult::new(
                seat["id"].as_str().unwrap(),
                seat["name"].as_str().unwrap(),
                seat["electorate"].as_u32().unwrap(),
                seat["turnout"].as_u32().unwrap(),
                winner,
                seat["result"].as_str().unwrap(),
                candidates
            );
            
            results.push(seat_result);
        }

        return Some(results);
    } else {
        println!("Error parsing file as JSON");
        return None;
    }
}