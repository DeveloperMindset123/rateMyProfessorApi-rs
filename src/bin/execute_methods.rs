// Shows methods examples
use tokio;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use predicates::prelude::*;
use rateMyProfessorApi_rs::methods::RateMyProfessor;

#[tokio::main]
pub async fn main() -> Result<()> {
    // constructor 1 example
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    println!("{rate_my_professor_instance:#?}");


    // constructor 2 example
    let mut rate_my_professor_instance_version2 = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    println!("newly instantiated object : {rate_my_professor_instance_version2:#?}\n\n");

    // setter example
    rate_my_professor_instance_version2.set_new_professor_and_college("Alejandro Crawford", "Baruch College");
    println!("{rate_my_professor_instance_version2:#?}\n");


    // instantiated method example
    let professor_list = rate_my_professor_instance_version2.get_professor_list_and_save("baruch_college_professor_list.json").await;
    println!("{professor_list:#?}");
    Ok(())
}