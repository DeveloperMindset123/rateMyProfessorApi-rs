mod experimental;
use experimental::*;        // wildcard placeholder
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RateMyProfessor {
    CollegeName : String,
    ProfessorName : Option<String>
}

impl RateMyProfessor {
    // this will specify no professor at the moment
    // constructor 1
    pub fn construct_college(college_name : &str) -> Self {
        RateMyProfessor {
            CollegeName : college_name.to_owned(),
            ProfessorName : Some("".to_owned())        // empty string placeholder
        }
    }

    // constructor 2
    pub fn construct_college_and_professor(college_name : &str, professor_name : &str) -> Self {
        RateMyProfessor {
            CollegeName : college_name.to_owned(),
            ProfessorName : Some(professor_name.to_owned())
        }
    }

    // method 1, retrieves college info
    // retrurn type should match search_school
    pub async fn get_college_info(&mut self) -> Result<Vec<SchoolSearch>> {
        experimental::search_school(&self.CollegeName).await
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("CUNY City College of New York");
    let data = rate_my_professor_instance.get_college_info().await?;
    println!("{data:?}");
    Ok(())
}