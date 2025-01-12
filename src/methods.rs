#[allow(unused_imports)]
#[allow(unused_doc_comments)]
use crate::features::*;        // wildcard placeholder
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio;
use predicates::prelude::*;

/// Primary Struct composed of two components : `CollegeName` and `ProfessorName`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RateMyProfessor {
    /// Constructor below will handle the `.to_owned()` method, simply pass in the string.
    pub CollegeName : String,               

    /// Will be stored as `Some("Name of Professor")`.
    pub ProfessorName : Option<String>      
}


impl RateMyProfessor {
    // this will specify no professor at the moment
    /// # Constructor 1 
    /// Takes in only 1 parameter, which is the name of the college. Name of professor will simply be set to a empty string and can be adjusted using the set_new_professor("name of professor") method.
    /// 
    /// ```rust
    /// // working code example
    /// use rateMyProfessorApi_rs::*;
    /// use anyhow::Result;
    /// use rateMyProfessorApi_rs::methods::RateMyProfessor;        // import struct
    /// 
    /// // while #[tokio::main] is not neccessary for this particular constructor since it's synchronous
    /// // It is still recommended since a few of the methods are synchronous
    /// // example WITHOUT [tokio::main] attribute
    /// fn main() {
    ///     let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    ///     println!("{rate_my_professor_instance:#?}");
    /// }
    /// 
    /// // example WITH [tokio::main] attribute
    /// [tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    ///     println!("{rate_my_professor_instance:#?}");    // check results
    /// 
    ///     // invoke one of the asynchornous functions that doesn't require Teacher's Name
    ///     let college_info = rate_my_professor_instance.get_college_info().await;
    ///     println!("{college_info:#?}");      // print out college info
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn construct_college(college_name : &str) -> Self {
        RateMyProfessor {
            CollegeName : college_name.to_owned(),
            ProfessorName : Some("".to_owned())        // empty string placeholder
        }
    }

    /// # Constructor 2 
    /// Takes in 2 parameters, first parameter is the name of the college and the second parameter is the name of the professor.
    /// 
    /// ```rust
    /// // example with constructor 2
    /// use tokio;
    /// use anyhow::Result;
    /// use rateMyProfessorApi_rs::methods::RateMyProfessor;
    /// 
    /// #[tokio::main]
    /// pub async fn main() -> Result<()> {
    ///     // constructor 2 example
    ///     let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    ///     println!("newly instantiated object : {rate_my_professor_instance:#?}\n\n");
    /// 
    ///     // we can retrieve information about college just like before
    ///     let college_info = rate_my_professor_instance.get_college_info().await;
    ///     println!("{college_info:#?}");
    /// 
    ///     // we can also retrieve information about the particular professor
    ///     // note that this function takes in a boolean flag named retrieve_all_result
    ///     //
    ///     // if retrieve_all_result is set to false, it will only refer to the professor (or professors if two professor shares the same name)
    ///     // and return that information
    ///     // in this case, the professor would be "Douglas Troeger" from "City College of New York"
    ///     let professor_summary_data = rate_my_professor_instance.get_teacher_summary(false).await?;
    ///     println!("{professor_summary_data:#?}");
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn construct_college_and_professor(college_name : &str, professor_name : &str) -> Self {
        RateMyProfessor {
            CollegeName : college_name.to_owned(),
            ProfessorName : Some(professor_name.to_owned())
        }
    }

    /// # Description
    /// - Retrieves college info, this method does not require name of a professor.
    /// 
    /// - The example for this has been provided within the constructor methods.
    pub async fn get_college_info(&mut self) -> Result<Vec<SchoolSearch>> {
        search_school(&self.CollegeName).await        // this function automatically handles this
    }

    /// # Description
    /// - Retrieve_all_result is a boolean flag that will specify whether user wants just the specific data or the entire data to be returned
    /// 
    /// - If retrieve_all_result is set to true, then any similar naming professor's data within the particular college will also be returned (Less Case Sensetive).
    /// 
    /// ```rust
    /// use tokio;
    /// use anyhow::Result;
    /// use rateMyProfessorApi_rs::methods::RateMyProfessor;
    /// 
    /// // previous example showcases what happens when the boolean flag is set to false
    /// // NOTE : this method requires specifying the name of the professor
    /// // You can either use RateMyProfessor::construct_college_and_professor("Name of college here", "Name of professor here")
    /// //
    /// // Or you can use set_new_professor after instantiating the object
    /// // RateMyProfessor::construct_college("Name of college here")
    /// // example below shows both approaches
    /// #[tokio::main]
    /// pub async fn main() -> Result<()> {
    /// 
    ///     // approach 1 : using the first constructor and setter method
    ///     let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    ///     println!("Object instance before setter method update : {rate_my_professor_instance:#?}");
    /// 
    ///     rate_my_professor_instance.set_new_professor("Ross Greenberg");         // in-place modification
    ///     println!("Object instance after setter method update : {rate_my_professor_instance:#?}");
    ///     
    ///     let teacher_summary = rate_my_professor_instance.get_teacher_summary(true).await?;
    ///     println!("{teacher_summary:#?}");
    /// 
    ///     // approach 2 : using the second constructor and directly invoking the method
    ///     let mut rate_my_professor_instance_version2 = RateMyProfessor::construct_college_and_professor("City College of New York", "Jie Wei");
    ///     println!("{rate_my_professor_instance_version2:#?}");
    /// 
    ///     let mut teacher_summary_version2 = rate_my_professor_instance_version2.get_teacher_summary(true).await?;
    /// 
    ///     // You may notice there's additional professor's information listed
    ///     // Due to the lack of case sensetivity since no filtering is being done on the data
    ///     println!("{teacher_summary_version2:#?}");      
    /// 
    ///     Ok(())     
    /// }
    /// 
    /// ```
    pub async fn get_teacher_summary(&mut self, retrieve_all_result : bool) -> Result<Vec<ProfessorRating>> {
        let mut result_data : Vec<ProfessorRating> = Vec::new();        // stores filtered results
        let mut rating_data_holder : Vec<ProfessorRating> = Vec::new();     // stores all results

        // check if the name of the profesosr is empty or not
        if self.ProfessorName == Some("".to_owned()) {
            eprintln!("You must first provide a name of a professor before attempting to get summary on a professor!.\n HINT : use the .set_new_professor('Name of Professor') to set the name of new professor before reattempting this method.");
            std::process::exit(1);
        } else {
            let schools = search_school(&self.CollegeName).await;
            let unwrap_professor_name = self.ProfessorName.clone().unwrap();
            if let Some(school) = schools.unwrap().first() {
                let school_id = &school.node.id;

                // pass in the school ID to search for the professor
                // search for the specific professors
                // then pass in their corresponding data into the get method
                // the search function takes in 2 parameters : name of professor and the corresponding school_id
                let professor_list = search_professors_at_school_id(&unwrap_professor_name, school_id).await;
                for professor in &professor_list?.clone() {

                    // get_professor_rating_at_school_id takes in two parameters, the name of the professor, which search_professors_at_school_id returns in the format of first and last name which needs to be merged back together

                    // as well as referencing the school_id (the type is &str)
                    let current_professor_rating = get_professor_rating_at_school_id(&format!("{} {}", professor.node.first_name, professor.node.last_name), &school_id).await.unwrap();
                    rating_data_holder.push(current_professor_rating);
                }
                
                for rating in &rating_data_holder {
                    // adds 2 layer of filtering
                    if unwrap_professor_name.to_owned() == rating.formatted_name && self.CollegeName == rating.college_name {
                        result_data.push(rating.clone());
                    } 
                }

                if result_data.is_empty() {
                    for rating in &rating_data_holder {
                        if unwrap_professor_name.clone().to_owned() == rating.formatted_name.clone() {
                            result_data.push(rating.clone())
                        }
                    }
                }
                result_data.sort_by(|a,b| a.partial_cmp(b).expect("Failed to sort the vector"));         // in-place sorting

                result_data.dedup();
            }
        }


        // logic based on boolean flag
        if retrieve_all_result {
            return Ok(rating_data_holder)
        } else {
            return Ok(result_data)
        }
    }

    /// # Description
    /// - This will do all the same functionality as get_teacher_summary but also save the resulting data within a JSON file.
    /// 
    // - Users can specify the file path they want in string format.

    /// NOTE : Function needs to be asynchronous, otherwise, data retrieval will not be successful.
    /// 
    /// - File will be created within the current working directory.
    /// 
    /// - Make sure to specify a JSON file for the file_name parameter (i.e. "some_json_file.json")
    /// 
    /// ```rust
    /// use tokio;
    /// use anyhow::Result;
    /// use rateMyProfessorApi_rs::methods::RateMyProfessor;
    /// 
    /// #[tokio::main]
    /// pub async fn main() -> Result<()> {
    ///     let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Jie Wei");
    ///     println!("{rate_my_professor_instance:#?}");
    ///
    ///     let mut teacher_summary = rate_my_professor_instance.get_teacher_summary_and_save(false, "Teacher_Summary.json").await?;
    ///     println!("{teacher_summary:#?}");

    ///     // Creates a file named "Teacher_Summary.json" within the current working directory.
    ///     // Content of the Json File:
    ///     //     [
    ///     //   {
    ///     //     "avgRating": 3.5,
    ///     //     "avgDifficulty": 3.1,
    ///     //     "wouldTakeAgainPercent": 46.6667,
    ///     //     "numRatings": 32,
    ///     //     "formattedName": "Jie Wei",
    ///     //     "department": "Computer Science",
    ///     //     "name": "City College of New York",
    ///     //     "link": "https://www.ratemyprofessors.com/professor/354797"
    ///     //   }
    ///     // ]
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_teacher_summary_and_save(&mut self, retrieve_all_result : bool, file_name : &str) -> Result<Vec<ProfessorRating>> {
        let result = self.get_teacher_summary(retrieve_all_result).await?;
        let (file, file_path) = create_file(file_name).await;
        save_data_to_file(file, &serde_json::to_string(&result).unwrap()).await;
        Ok(result)
    }

    /// # Description
    /// - Simple setter that will update the current professor
    /// synchronous method is sufficieint.
    /// 
    /// - Function doesn't return anything, performs in-place modification.
    pub fn set_new_professor(&mut self, professor_name : &str) {
        self.ProfessorName = Some(professor_name.to_owned());
    }

    /// # Description
    /// - Similar to `set_new_professor("name of professor")`.
    /// 
    /// - A simple setter that will update the current college with the new college.
    pub fn set_new_college(&mut self, college_name : &str) {
        self.CollegeName = college_name.to_owned();
    }

    /// # Description:
    /// - Invoke `self.set_new_professor()` and `self.set_new_college()` : this is a nested function.
    /// - Helpful if you want to update both the professor and college name simultaneously.
    pub fn set_new_professor_and_college(&mut self, professor_name : &str, college_name : &str) {
        self.set_new_college(college_name);
        self.set_new_professor(professor_name);
    }

    /// # Description
    /// Returns all of the comments corresponding to the current professor.
    /// Array of objects is returned, the object contains the following information:
    /// - comment (comment made by a particular student)
    /// - class name (class which the professor teaches)
    /// - date (timestamp for when this comment entry has been made in UTC format)
    /// - rating_tags (any relevant rating tags i.e. Gives good feedback--Clear grading criteria--Would take again)
    /// - difficulty (floating point value representing the difficulty of the course between a scale of 1.0-5.0, 5 being the most difficult and 1 being the easiest).
    /// - grade (grade the corresponding student got within the particular course)
    /// - would_take_again (boolean value representing whether a particular student would want to retake the course again or not)
    /// 
    /// ```rust
    /// 
    /// // add the following within the main function from above (or any asynchronous function)
    /// let rate_my_professor_instance.get_professor_comments().await?;
    /// println!("{professor_comments:#?}");
    /// 
    /// // sample output:
    /// // [ 
    /// //     ProfessorComments {
    /// //          comment: "Very interesting course. Teaches the subject very well and makes the class fun. Going to class and taking notes will help you a lot for the quiz,midterm,and final. Find a good group (great programmers) and start it as early as you can. Lab quiz will be based on the handouts from your lab TA.",
    /// //          class_name: "CS322",
    /// //          date: "2016-12-16 21:20:07 +0000 UTC",
    /// //          rating_tags: "Clear grading criteria--Amazing lectures--GROUP PROJECTS",
    /// //          difficulty: 3.0,
    /// //          grade: "Not Available",
    /// //          would_take_again: true,
    /// //     }, 
    /// //     ProfessorComments {
    /// //         comment: "Very good lectures. You understand the material if you follow through.",
    /// //         class_name: "CSCII100",
    /// //         date: "2017-12-06 00:15:27 +0000 UTC",
    /// //         rating_tags: "Gives good feedback--Clear grading criteria--Would take again",
    /// //         difficulty: 3.0,
    /// //         grade: "A",
    /// //         would_take_again: true,
    /// //     },
    /// // ... additional data
    /// // ]
    /// ```
    pub async fn get_professor_comments(&mut self) -> Result<Vec<ProfessorComments>>  {
        // calls on two function
        // step 1 : invoke search_professor_id 
        // step 2 : invoke search_professor_comments
        let professor_id = search_professor_id(&self.ProfessorName.clone().unwrap(), &self.CollegeName).await?;
        let professor_comments = search_professor_comments(professor_id).await?;

        Ok(professor_comments)
    }

    /// # Description:
    /// - Invokes self.get_professor_comments, the content is then saved within a json file specified within the `file_name` parameter. This method works similar to get_teacher_summary_and_save, except the content that is returned is different.
    /// 
    /// - Alongside the saved file, the fetched data is also returned as a return value in array format.
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
        save_data_to_file(professor_comments_file, &professor_comments_vector_unwrapped).await;

        Ok(professor_comments_data)
    }

    /// # Description:
    /// - This method will retrieve the list of all the professors corresponding to the current college, does not require the name of the current professor.
    /// 
    /// - Helpful in the event that there's a particular college your interested in and want to retrieve a list of professors to gather data about them.
    /// 
    /// ```rust
    /// let mut rate_my_professor_instance = RateMyProfessor::construct_college("CUNY Queens College");
    /// let mut list_of_professors = rate_my_professor_instance.get_professor_list().await?;
    /// println!("{list_of_professors:#?}");
    /// 
    /// // sample output:
    /// // [ ProfessorList {
    /// //     id: Some(
    /// //         "VGVhY2hlci0xOTgxNzY0",
    /// //     ),
    /// //     legacy_id: Some(
    /// //         "VGVhY2hlci0xOTgxNzY0",
    /// //     ),
    /// //     first_name: Some(
    /// //         "Rebecca",
    /// //     ),
    /// //     last_name: Some(
    /// //         "Nelson",
    /// //     ),
    /// //     department: Some(
    /// //         "Theater",
    /// //     ),
    /// //     avg_rating: Some(
    /// //         4.8,
    /// //     ),
    /// //     num_rating: Some(
    /// //         24,
    /// //     ),
    /// //     avg_difficulty: Some(
    /// //         1.4,
    /// //     ),
    /// // },
    /// // ProfessorList {
    /// //     id: Some(
    /// //         "VGVhY2hlci00MTQ3MTM=",
    /// //     ),
    /// //     legacy_id: Some(
    /// //         "VGVhY2hlci00MTQ3MTM=",
    /// //     ),
    /// //     first_name: Some(
    /// //         "Donald",
    /// //     ),
    /// //     last_name: Some(
    /// //         "Scott",
    /// //     ),
    /// //     department: Some(
    /// //         "History",
    /// //     ),
    /// //     avg_rating: Some(
    /// //         2.7,
    /// //     ),
    /// //     num_rating: Some(
    /// //         24,
    /// //     ),
    /// //     avg_difficulty: Some(
    /// //         2.9,
    /// //     ),
    /// // },
    /// // ... additional data]
    /// 
    /// ```
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

    /// This function will not only fetch and return the list of professors, but the returned data will be saved to the corresponding json file within the current working directory based on the name specified for the `file_name` parameter.
    pub async fn get_professor_list_and_save(&mut self, file_name : &str) -> Result<Vec<ProfessorList>> {
        let professor_list_data = self.get_professor_list().await?;
        let (created_file, _file_path) = create_file(file_name).await;
        save_data_to_file(created_file, &serde_json::to_string(&professor_list_data).unwrap()).await;
        Ok(professor_list_data)
    }
}


/// working examples
/// ```rust
/// #[tokio::main]
/// pub async fn main() -> Result<()> {

///     // constructor 2 example
///     let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
///     println!("newly instantiated object : {rate_my_professor_instance:#?}\n\n");

///     // setter example
///     rate_my_professor_instance.set_new_professor_and_college("Alejandro Crawford", "Baruch College");
///     println!("{rate_my_professor_instance:#?}\n");


///     // instantiated method example
///     let professor_list = rate_my_professor_instance.get_professor_list_and_save("baruch_college_professor_list.json").await;
///     println!("{professor_list:#?}");
///     Ok(())
/// }
/// ```
#[tokio::main]
pub async fn main() -> Result<()> {

    // constructor 2 example
    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    println!("newly instantiated object : {rate_my_professor_instance:#?}\n\n");

    // setter example
    rate_my_professor_instance.set_new_professor_and_college("Alejandro Crawford", "Baruch College");
    println!("{rate_my_professor_instance:#?}\n");


    // instantiated method example
    let professor_list = rate_my_professor_instance.get_professor_list_and_save("baruch_college_professor_list.json").await;
    println!("{professor_list:#?}");
    Ok(())
}