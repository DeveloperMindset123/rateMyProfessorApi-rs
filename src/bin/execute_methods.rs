// Shows methods examples
use tokio;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use predicates::prelude::*;
use rateMyProfessorApi_rs::methods::RateMyProfessor;

#[tokio::main]
pub async fn main() -> Result<()> {
    // constructor 1 example
    // let mut rate_my_professor_instance = RateMyProfessor::construct_college("CUNY Queens College");
    // // println!("Object instance before setter method update : {rate_my_professor_instance:#?}");

    // let mut college_summary = rate_my_professor_instance.get_college_info().await?;
    // println!("{college_summary:?}");

    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");

    let professor_comments = rate_my_professor_instance.get_professor_comments().await?;

    println!("{professor_comments:#?}");

    // let mut teacher_summary = rate_my_professor_instance.get_teacher_summary_and_save(false, "Hamed_Fazli.json").await?;
    // println!("{teacher_summary:#?}");
    // let mut list_of_professors = rate_my_professor_instance.get_professor_list().await?;
    // println!("{list_of_professors:#?}");
    // rate_my_professor_instance.set_new_professor("Ross Greenberg");         // in-place modification
    // println!("Object instance after setter method update : {rate_my_professor_instance:#?}");

    // let teacher_summary = rate_my_professor_instance.get_teacher_summary(true).await?;
    // println!("{teacher_summary:#?}");

    // let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Jie Wei");
    // println!("{rate_my_professor_instance:#?}");

    // let mut teacher_summary = rate_my_professor_instance.get_teacher_summary_and_save(false, "Teacher_Summary.json").await?;
    // println!("{teacher_summary:#?}");

    // let mut professor_comments = rate_my_professor_instance.get_professor_comments().await?;
    // println!("{professor_comments:#?}");



    // sample output:
    // [ 
    //     ProfessorComments {
    //          comment: "Very interesting course. Teaches the subject very well and makes the class fun. Going to class and taking notes will help you a lot for the quiz,midterm,and final. Find a good group (great programmers) and start it as early as you can. Lab quiz will be based on the handouts from your lab TA.",
    //          class_name: "CS322",
    //          date: "2016-12-16 21:20:07 +0000 UTC",
    //          rating_tags: "Clear grading criteria--Amazing lectures--GROUP PROJECTS",
    //          difficulty: 3.0,
    //          grade: "Not Available",
    //          would_take_again: true,
    //     }, 
    //     ProfessorComments {
    //         comment: "Very good lectures. You understand the material if you follow through.",
    //         class_name: "CSCII100",
    //         date: "2017-12-06 00:15:27 +0000 UTC",
    //         rating_tags: "Gives good feedback--Clear grading criteria--Would take again",
    //         difficulty: 3.0,
    //         grade: "A",
    //         would_take_again: true,
    //     },
    // ]

    // invoke one of the asynchornous functions that doesn't require Teacher's Name
    // let college_info = rate_my_professor_instance.get_college_info().await;
    // println!("{college_info:#?}");

    // constructor 2 example
    // let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    // println!("newly instantiated object : {rate_my_professor_instance:#?}\n\n");

    // we can retrieve information about college just like before
    // let college_info = rate_my_professor_instance.get_college_info().await;
    // println!("{college_info:#?}");

    // we can also retrieve information about the particular professor
    // note that this function takes in a boolean flag named retrieve_all_result
    //
    // if retrieve_all_result is set to false, it will only refer to the professor (or professors if two professor shares the same name)
    // and return that information
    // in this case, the professor would be "Douglas Troeger" from "City College of New York"
    // let professor_summary_data = rate_my_professor_instance.get_teacher_summary(true).await?;
    // println!("{professor_summary_data:#?}");      

    // setter example
    // rate_my_professor_instance_version2.set_new_professor_and_college("Alejandro Crawford", "Baruch College");
    // println!("{rate_my_professor_instance_version2:#?}\n");


    // // instantiated method example
    // let professor_list = rate_my_professor_instance_version2.get_professor_list_and_save("baruch_college_professor_list.json").await;
    // println!("{professor_list:#?}");
    Ok(())
}

async fn test_constructor_methods() -> Result<()> {
    // constructor 1 example
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    println!("{rate_my_professor_instance:#?}");

    let comparison_struct = RateMyProfessor {
        CollegeName : "Queens College".to_owned(),
        ProfessorName : Some("".to_owned())
    };    

    assert_eq!(rate_my_professor_instance, comparison_struct);

    // constructor 2 example
    let mut rate_my_professor_instance_version2 = RateMyProfessor::construct_college_and_professor("City College of New York", "Douglas Troeger");
    println!("newly instantiated object : {rate_my_professor_instance_version2:#?}\n\n");

    Ok(())
}

#[tokio::test]
async fn test_asynchronous_functions() -> Result<()> {
    test_constructor_methods().await;
    Ok(())
}