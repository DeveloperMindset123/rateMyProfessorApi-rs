# rate_my_professor_rs
## Overview
A lightweight API wrapper around [Rate My Professor](https://www.ratemyprofessors.com/). Complex API calls and graphql queries have been abstracted away and simplified into the list of following features (with examples provided below):
- **Retrieve College Summary** : Get information related to a particular college. Such as the list of departments, loction of the college, condition of the campus, safety and satisfaction level and more.

- **Retrieve Summary About a Particular Professor** : Retrieves information about the professor, information such as the number of ratings that the professor has recieved, the department from which the particular professor is from, original link to the Rate My Professor Website. 
    - In addition, information such as average rating, average difficulty, and percentage of student likely to retake this particular professor.

- **Retrieve List of Professors** : If an user wants to obtain information about the number of professors currently recorded within Rate My Professor, there's a method that can retrieve the list of all the professors for a particular college.
    - **NOTE** : New professors may not always be recorded within Rate My Professor, esepcially if they happen to be graduate students or their first semester teaching, this information can always be verified on the actual college website.

- **Retrieve Comments** : Users can retrieve list of all the comments that students has made for a particular professor wtihin a particular college. 

[API Documentation](https://docs.rs/rate_my_professor_api_rs/0.1.3/rateMyProfessorApi_rs/) : To get better understanding of implementation details.

### Examples:
**NOTE** : There are more examples that can be found within the API Documentation.

#### Installation:
- While not all of the dependancies are needed for the methods module, if you do wish to use the features module, some additional dependancies such as serde and reqwest will be needed.
- The list below covers all the dependancies needed.
```toml
[dependancies]
rate_my_professor_api_rs = "0.1.3"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.12", features = ["json"] }
anyhow = "1.0.95"
tokio-macros = "2.5.0"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.135"
```

#### Code Examples
- There's two different constructors. The first constructor only requires specifying the name of the college itself. 
- Example using first constructor shown below:
```rust
// working code example
use tokio;
use anyhow::Result;
use rateMyProfessorApi_rs::methods::RateMyProfessor;        // import struct
 
// while #[tokio::main] is not neccessary for this particular constructor since it's synchronous
// It is still recommended since a few of the methods are synchronous
// example WITHOUT [tokio::main] attribute
// comment this function out if you wish to use the async main()
fn main() {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    println!("{rate_my_professor_instance:#?}");
}
 
// example WITH [tokio::main] attribute
// comment this function out if you wish to use the default main()
#[tokio::main]
async fn main() -> Result<()> {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college("Queens College");
    println!("{rate_my_professor_instance:#?}");    // check results
 
    // invoke one of the asynchornous functions that doesn't require Teacher's Name
    let college_info = rate_my_professor_instance.get_college_info().await;
    println!("{college_info:#?}");      // print out college info
    
    // retrieves list of all the professors
    // currently recorded within rate my professor
    //
    // this method can take 2-3 seconds to execute
    let professor_list = rate_my_professor_instance.get_professor_list().await?;
    println!("{professor_list:#?}");
    
    // sample output for list of professors
    // [
// ProfessorList {
//         id: Some(
//             "VGVhY2hlci0xMDMzNzQ0",
//         ),
//         legacy_id: Some(
//             "VGVhY2hlci0xMDMzNzQ0",
//         ),
//         first_name: Some(
//             "John",
//         ),
//         last_name: Some(
//             "Wigglesworth",
//         ),
//         department: Some(
//             "Philosophy",
//         ),
//         avg_rating: Some(
//             4.8,
//         ),
//         num_rating: Some(
//             24,
//         ),
//         avg_difficulty: Some(
//             1.4,
//         ),
//     },
//     ProfessorList {
//         id: Some(
//             "VGVhY2hlci0xMDk3OTA1",
//         ),
//         legacy_id: Some(
//             "VGVhY2hlci0xMDk3OTA1",
//         ),
//         first_name: Some(
//             "Roxanne",
//         ),
//         last_name: Some(
//             "Nedelcu",
//         ),
//         department: Some(
//             "Psychology",
//         ),
//         avg_rating: Some(
//             2.9,
//         ),
//         num_rating: Some(
//             24,
//         ),
//         avg_difficulty: Some(
//             3.4,
//         ),
//     },
// ]
 
    Ok(())
}

```

- Example using the second constructor:
```rust
use tokio;
use anyhow::Result;
use rateMyProfessorApi_rs::methods::RateMyProfessor;
 
#[tokio::main]
pub async fn main() -> Result<()> {
    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Jie Wei");
    println!("{rate_my_professor_instance:#?}");

// This method lets you not only retrieve the summary 
// But also save it on a corresponding json file
//
// In this case, it has been saved within "Teacher_Summary.json"
    let mut teacher_summary = rate_my_professor_instance.get_teacher_summary_and_save(false, "Teacher_Summary.json").await?;
    println!("{teacher_summary:#?}");
    // Creates a file named "Teacher_Summary.json" within the current working directory.
    // Content of the Json File:
    //     [
    //   {
    //     "avgRating": 3.5,
    //     "avgDifficulty": 3.1,
    //     "wouldTakeAgainPercent": 46.6667,
    //     "numRatings": 32,
    //     "formattedName": "Jie Wei",
    //     "department": "Computer Science",
    //     "name": "City College of New York",
    //     "link": "https://www.ratemyprofessors.com/professor/354797"
    //   }
    // ]
    Ok(())
}
```

The following example shows what typical comments would look like from a particular professor:
```rust
use tokio;
use anyhow::Result;
use rateMyProfessorApi_rs::methods::RateMyProfessor;

// add the following within the main function from above (or any asynchronous function)
#[tokio::main]
async fn main() {

    let mut rate_my_professor_instance = RateMyProfessor::construct_college_and_professor("City College of New York", "Jie Wei");

    let mut professor_comments = rate_my_professor_instance.get_professor_comments().await.unwrap();
    println!("{professor_comments:#?}");
}
 
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
// ... additional data
// ]

```
### Getting Help
- If the [API Documentation](https://docs.rs/rate_my_professor_api_rs/0.1.3/rateMyProfessorApi_rs/) doesn't help and you happen to be stuck on something, there's also examples within the **bin** folder containing executable code.

- However, if that doesn't work or solve your problem, or if there's a feature request you would like to make, you can reach out to me in the following:

**Gmail**: dasa60196@gmail.com

**Discord** : the1sand0s (Just send a friend request and message me)
### License
This project is licensed under the MIT License.