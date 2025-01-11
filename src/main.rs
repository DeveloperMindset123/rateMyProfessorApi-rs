mod experimental;
use experimental::*;        // wildcard placeholder
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use predicates::prelude::*;

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
        experimental::search_school(&self.CollegeName).await        // this function automatically handles this
    }

    // NOTE : would be best to rename this as "search_professor_by_name"

    // main return type : Result<Vec<ProfessorRating>>
    // retrieve_all_result is a boolean flag that will specify whether user wants just the specific data or the entire data to be returned
    pub async fn get_teacher_summary(&mut self, retrieve_all_result : bool) -> Result<Vec<ProfessorRating>> {
        // TODO : needs to handle null values in the event that ProfessorName is null
        // NOTE : all inner functions must be set to await method as well, otherwise, data will not be successful during retrieval
        // should return a set of values
        let mut result_data : Vec<ProfessorRating> = Vec::new();        // stores filtered results
        let mut rating_data_holder : Vec<ProfessorRating> = Vec::new();     // stores all results

        // check if the name of the profesosr is empty or not
        // TODO : test if this can bypass the return type explicitly
        if self.ProfessorName == Some("".to_owned()) {
            eprintln!("You must first provide a name of a professor before attempting to get summary on a professor!.\n HINT : use the .set_new_professor('Name of Professor') to set the name of new professor before reattempting this method.");
            std::process::exit(1);
        } else {
            // assuming professor name has indeed been provided
            // I also need to retrieve the school id from the search_school function

            // search for the school once again
            let schools = experimental::search_school(&self.CollegeName).await;
            let unwrap_professor_name = self.ProfessorName.clone().unwrap();
            // extract the school ID
            if let Some(school) = schools.unwrap().first() {
                // println!("Found school : {} in {}, {}", school.node.name, school.node.city, school.node.city);
                let school_id = &school.node.id;

                // pass in the school ID to search for the professor
                // search for the specific professors
                // then pass in their corresponding data into the get method
                // the search function takes in 2 parameters : name of professor and the corresponding school_id
                let professor_list = experimental::search_professors_at_school_id(&unwrap_professor_name, school_id).await;


                // NOTE : Result<> does not have Debug trait, it must be unwrapped in order to print it out successfully.
                // println!("\n\nsearch result of professor : {:?}", professor_list.unwrap());        // for testing

                // iterate over the professor list

                for professor in &professor_list?.clone() {

                    // get_professor_rating_at_school_id takes in two parameters, the name of the professor, which search_professors_at_school_id returns in the format of first and last name which needs to be merged back together

                    // as well as referencing the school_id (the type is &str)
                    let current_professor_rating = experimental::get_professor_rating_at_school_id(&format!("{} {}", professor.node.first_name, professor.node.last_name), &school_id).await.unwrap();

                    // println!("{:#?}", current_professor_rating);
                    rating_data_holder.push(current_professor_rating);

                }
                
                // println!("{rating_data_holder:#?}");

                for rating in &rating_data_holder {
                    // adds 2 layer of filtering
                    if unwrap_professor_name.to_owned() == rating.formatted_name && self.CollegeName == rating.college_name {
                        result_data.push(rating.clone());
                    } 
                    // else {
                    //     if result_data.is_empty() {
                    //         if unwrap_professor_name.to_owned() == rating.formatted_name {
                    //             result_data.push(rating.clone());
                    //         }
                    //     }
                    // }
                }

                // check and verify if the hard filtering didn't work
                // then we apply a single layer of filtering instead
                if result_data.is_empty() {
                    for rating in &rating_data_holder {
                        if unwrap_professor_name.clone().to_owned() == rating.formatted_name.clone() {
                            result_data.push(rating.clone())
                        }
                    }
                }
                result_data.sort_by(|a,b| a.partial_cmp(b).expect("Failed to sort the vector"));         // in-place sorting

                // NOTE : this isn't fully working.
                result_data.dedup();        // inplace modification removing duplicate consecutive elements
                // let set : std::collections::BTreeSet<_> = result_data.drain(..).collect();
                // for x in set {
                //     if let Some(last) = result_data.last() {
                //         if predicate(last, &x) { continue; }
                //     }
                //     result_data.push(x);
                // }

                // let set: BTreeSet<_> = vec.drain(..).collect();
                // for x in set {
                // // data comes in in sorted order so you can further
                // // process adjacenct elements like this
                //     if let Some(last) = vec.last() {
                //         if predicate(last, &x) { continue; }
                //         }
                //     vec.push(x);
                //     }
                    println!("{result_data:#?}");
            }
        }

        // NOTE the structure of TeacherNode (Since it's nested) --> for reference to create null data handler
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TeacherNode {
//     pub __typename: String,   // unused variable

//     #[serde(rename="avgDifficulty")]            // formatting, matches the current value to what the returned object type would be using this rust attribute
//     pub avg_difficulty: f64,

//     #[serde(rename="avgRating")]
//     pub avg_rating: f64,
//     pub department: String,

//     #[serde(rename="firstName")]
//     pub first_name: String,
//     pub id: String,

//     #[serde(rename="isSaved")]
//     pub is_saved: bool,

//     #[serde(rename="lastName")]
//     pub last_name: String,

//     #[serde(rename="legacyId")]
//     pub legacy_id: i64,

//     #[serde(rename="numRatings")]
//     pub num_ratings: i32,
//     pub school: School,

//     #[serde(rename="wouldTakeAgainPercent")]
//     pub would_take_again_percent: f64,
// }

// previous struct is a dependency on this struct
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TeacherSearch {
//     pub cursor: String,
//     pub node: TeacherNode,
// }

        if retrieve_all_result {
            return Ok(rating_data_holder)
        } else {
            return Ok(result_data)
        }
    }
}


// this should be moved to "examples" directory
#[tokio::main]
pub async fn main() -> Result<()> {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    // let data = rate_my_professor_instance.get_college_info().await?;    // tested:worked

    let mut get_teacher_summary = rate_my_professor_instance.get_teacher_summary(false).await?;
    println!("{get_teacher_summary:#?}");
    Ok(())
}