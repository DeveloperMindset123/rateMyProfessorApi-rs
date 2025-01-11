mod experimental;
use experimental::*;        // wildcard placeholder
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use predicates::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RateMyProfessor {
    pub CollegeName : String,
    pub ProfessorName : Option<String>
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


        // logic based on boolean flag
        if retrieve_all_result {
            return Ok(rating_data_holder)
        } else {
            return Ok(result_data)
        }
    }

    // this will do all the same functionality as get_teacher_summary but also save the resulting data
    // user can specify the file path they want in string format
    // everything needs to be asynchronous, otherwise, data retrieval will not be successful
    // will create the file in the immediate directory
    pub async fn get_teacher_summary_and_save(&mut self, retrieve_all_result : bool, file_name : &str) -> Result<Vec<ProfessorRating>> {
        let result = self.get_teacher_summary(retrieve_all_result).await?;
        let (file, file_path) = experimental::create_file(file_name).await;
        experimental::save_data_to_file(file, &serde_json::to_string(&result).unwrap()).await;
        Ok(result)
    }

    // simple setter that will update the current professor
    // synchronous method is sufficieint
    // function shouldn't return anything, should simply save
    pub fn set_new_professor(&mut self, professor_name : &str) {
        self.ProfessorName = Some(professor_name.to_owned());
    }

    pub fn set_new_college(&mut self, college_name : &str) {
        self.CollegeName = college_name.to_owned();
    }

    // invoke self.set_new_professor() and self.set_new_college() : this is a higher order function
    pub fn set_new_professor_and_college(&mut self, professor_name : &str, college_name : &str) {
        self.set_new_college(college_name);
        self.set_new_professor(professor_name);
    }

    // actual return type : Result<Vec<ProfessorComments>>
    pub async fn get_professor_comments(&mut self) -> Result<Vec<ProfessorComments>>  {
        // need to call on two function
        // step 1 : invoke search_professor_id 
        // step 2 : invoke search_professor_comments
        let professor_id = search_professor_id(&self.ProfessorName.clone().unwrap(), &self.CollegeName).await?;
        let professor_comments = search_professor_comments(professor_id).await?;
        // println!("{professor_comments:#?}");     // tested:worked

        Ok(professor_comments)
    }

    // invoke self.get_professor_comments() and save it
    pub async fn get_professor_comments_and_save(&mut self, file_name : &str) -> Result<Vec<ProfessorComments>> {
        let professor_comments_data = self.get_professor_comments().await?;
        let (professor_comments_file, _professor_comments_file_path) = create_file(file_name).await;
        let professor_comments_vector_wrapped = serde_json::to_string(&professor_comments_data.clone());

        if professor_comments_vector_wrapped.is_err() {
            println!("Error, failed to serialize data : {}", professor_comments_vector_wrapped.unwrap_err());
            std::process::exit(1);
        } 
        // if we attempt to unwrap null data, compiler will panic
        let professor_comments_vector_unwrapped = professor_comments_vector_wrapped.unwrap();
        // println!("unwrapped data : {professor_comments_vector_unwrapped:?}");
        save_data_to_file(professor_comments_file, &professor_comments_vector_unwrapped).await;

        Ok(professor_comments_data)
    }

    pub async fn get_professor_list(&mut self) -> Result<Vec<ProfessorList>> {
        // retrieve the college id
        let colleges = search_school(&self.CollegeName).await;
        let mut college_id : String = "".to_owned();     // initialize empty string to store the data
        if let Some(college) = colleges.unwrap().first() {
            college_id = college.node.id.to_owned(); // ???
        }

        // after updating the value for the collge id, call on the college to retrieve professor list
        
        Ok(get_professor_list_by_school(&college_id).await?)
    }

    pub async fn get_professor_list_and_save(&mut self, file_name : &str) -> Result<Vec<ProfessorList>> {
        let professor_list_data = self.get_professor_list().await?;
        let (created_file, _file_path) = create_file(file_name).await;
        save_data_to_file(created_file, &serde_json::to_string(&professor_list_data).unwrap()).await;
        Ok(professor_list_data)
    }
}


// this should be moved to "examples" directory
#[tokio::main]
pub async fn main() -> Result<()> {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");

    // let mut get_teacher_summary = rate_my_professor_instance.get_teacher_summary_and_save(true, "all_teacher_data.json").await?;        // tested : worked
    // println!("{get_teacher_summary:#?}");
    rate_my_professor_instance.set_new_professor_and_college("Alejandro Crawford", "Baruch College");
    // println!("{rate_my_professor_instance:#?}");
    // println!("{:#?}",rate_my_professor_instance.get_professor_comments().await?);
    // println!("{:#?}",rate_my_professor_instance.get_professor_comments_and_save("professor_comments_data.json").await?);     // tested and worked
    // println!("{:#?}",rate_my_professor_instance.get_professor_list().await?);
    rate_my_professor_instance.get_professor_list_and_save("baruch_college_professor_list.json").await;
    Ok(())
}