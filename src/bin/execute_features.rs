use tokio;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use predicates::prelude::*;
use rateMyProfessorApi_rs::features::*;


#[tokio::main]
async fn main() -> Result<()> {
    let retrieved_professor_id : ProfessorId = search_professor_id("Jie Wei","CUNY City College of New York").await.unwrap();
    search_professor_comments(retrieved_professor_id).await?;

    let schools = search_school("CUNY City College of New York").await?;
    
    if let Some(school) = schools.first() {
        println!("Found school: {} in {}, {}", 
            school.node.name, 
            school.node.city, 
            school.node.state
        );
        
        // Then search for professors at that school
        // this is the correct school id
        let school_id = &school.node.id;
        let professor_list_returned = get_professor_list_by_school(school_id).await?;
        
    }

    Ok(())
}