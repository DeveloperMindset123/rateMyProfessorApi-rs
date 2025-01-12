// not a module
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_imports)]
#[allow(unused_doc_comments)]
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::{fs,io::Write};
use filepath::FilePath;
use std::path::PathBuf;
use std::any::type_name;
use core::cmp::Ord;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct School {
    pub id: String,   
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeacherNode {
    pub __typename: String,   // unused variable

    #[serde(rename="avgDifficulty")]            
    pub avg_difficulty: f64,

    #[serde(rename="avgRating")]
    pub avg_rating: f64,
    pub department: String,

    #[serde(rename="firstName")]
    pub first_name: String,
    pub id: String,

    #[serde(rename="isSaved")]
    pub is_saved: bool,

    #[serde(rename="lastName")]
    pub last_name: String,

    #[serde(rename="legacyId")]
    pub legacy_id: i64,

    #[serde(rename="numRatings")]
    pub num_ratings: i32,
    pub school: School,

    #[serde(rename="wouldTakeAgainPercent")]
    pub would_take_again_percent: f64,
}

const API_LINK: &str = "https://www.ratemyprofessors.com/graphql";      // base URL

// graphql query to get teacher rating
// this query should be executed after retrieving the teacher id
const TEACHER_COMMENTS : &str = r#"
query TeacherRatingsPageQuery($id: ID!) {
        node(id: $id) {
            __typename
            ... on Teacher {
                firstName
                lastName
                department
                ratings(first: 50) {
                    edges {
                        node {
                            comment
                            class
                            date
                            helpfulRating
                            difficultyRating
                            grade
                            wouldTakeAgain
                            ratingTags
                        }
                    }
                }
            }
        }
    }
"#;

const GET_TEACHER_ID_QUERY : &str = r#"
  query TeacherSearchResultsPageQuery(
        $query: TeacherSearchQuery!
        $schoolID: ID
        $includeSchoolFilter: Boolean!
    ) {
        search: newSearch {
            teachers(query: $query, first: 1) {
                edges {
                    node {
                        id
                        firstName
                        lastName
                    }
                }
            }
        }
        school: node(id: $schoolID) @include(if: $includeSchoolFilter) {
            __typename
            ... on School {
                name
            }
            id
        }
    }
"#;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfessorComments {
  pub comment : String,
  pub class_name : String,
  pub date : String,
  pub rating_tags : String,
  pub difficulty : f64,
  pub grade : String,
  pub would_take_again : bool
}

// returns ProfessorComments wrapped around Result
// get all comments for a specific professor based on teacher ID
pub async fn search_professor_comments(professorID : ProfessorId) -> Result<Vec<ProfessorComments>> {
  let professor_id : String = professorID.Id;
  let client = reqwest::Client::new();
  let payload = serde_json::json!({
    "query" : TEACHER_COMMENTS,
    "variables" : {"id" : professor_id}
  });

  // make the post request
  let response = client.post(API_LINK).headers(get_headers()).json(&payload).send().await?;

  if !response.status().is_success() {
    return Err(anyhow::anyhow!("Network response from RMP not OK"));
  }

  let mut comments_data : serde_json::Value = response.json().await.unwrap();
  let mut comments_subsection = comments_data["data"]["node"]["ratings"]["edges"].clone();
  let length = get_json_length(&comments_subsection);
  let mut ProfessorCommentsVector : Vec<ProfessorComments> = Vec::with_capacity(length.clone());

  // TODO : Save returned data to a JSON file as well for cleanliness
  for index in 0..length {
    // example of how to retrieve the comments
    let comments_data : String = serde_json::from_str(&comments_subsection[index]["node"]["comment"].to_string())?;

    let would_take_again : &serde_json::Value = &comments_subsection[index]["node"]["wouldTakeAgain"];

    let extracted_comments_data : String = serde_json::from_str(&comments_subsection[index]["node"]["comment"].to_string())?;

    let extracted_grade : String = serde_json::from_str(&comments_subsection[index]["node"]["grade"].to_string())?;

    let extracted_date : String = serde_json::from_str(&comments_subsection[index]["node"]["date"].to_string())?;

    let extracted_rating_tags : String = serde_json::from_str(&comments_subsection[index]["node"]["ratingTags"].to_string())?;

    let extracted_difficulty : f64 = serde_json::from_str(&comments_subsection[index]["node"]["difficultyRating"].to_string())?;
    
    // construct the struct
    let professor_comment_instance = ProfessorComments {
      comment : extracted_comments_data,
      class_name : serde_json::from_str(&comments_subsection[index]["node"]["class"].to_string())?,
      date : extracted_date,
      rating_tags : if extracted_rating_tags == "".to_owned() { "N/A".to_owned() } else { extracted_rating_tags },
      difficulty : extracted_difficulty,
      grade : if extracted_grade == "".to_owned() { "Not Available".to_owned()} else { extracted_grade },

      would_take_again : if comments_subsection[index]["node"]["wouldTakeAgain"] == serde_json::Value::Null { false} else {true}
    };
    ProfessorCommentsVector.push(professor_comment_instance);
  }
  
  Ok(ProfessorCommentsVector)
}

// retrieve the length of the returned data value using the match operator
// function only handles returned datatype from serde_json that are of Array and Object type
pub fn get_json_length(value : &serde_json::Value) -> usize {
  match value {
    serde_json::Value::Array(arr) => arr.len(),
    serde_json::Value::Object(obj) => obj.len(),
    _ => 0
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfessorId {
  pub Id : String
}
// returns the id of the professor given the professor name and school name 
pub async fn search_professor_id(professor_name : &str, school_name : &str) -> Result<ProfessorId> {
  
  // writting null in graphql is equivalent to writting None in python
  let school_id = get_school_id(school_name).await;
  let client = reqwest::Client::new();
  let variables = serde_json::json!({
    "query" : {
      "text" : professor_name,
      "schoolID" : school_id,
      "fallback" : true,
      "departmentID" : null   
    },
    "schoolID" : school_id,
    "includeSchoolFilter" : true
  });

  let payload = serde_json::json!({
    "query" : GET_TEACHER_ID_QUERY,
    "variables" : variables
  });

  let response = client
                  .post(API_LINK)
                  .headers(get_headers())
                  .json(&payload)
                  .send()
                  .await?;

  // error handler logic
  if !response.status().is_success() {
    return Err(anyhow::anyhow!("Network response from RMP not OK"));
  }

  // retrieve and extract json data
  let search_result : serde_json::Value = response.json().await?;

  // correct attempt at retrieving the id
  let sample_id = search_result["data"]["search"]["teachers"]["edges"][0]["node"]["id"].clone().to_string();

  let sample_id_string : &str= serde_json::from_str(&sample_id).unwrap();
  println!("sample professor id is : {:?}", sample_id_string);
  Ok(ProfessorId {
    Id : sample_id_string.to_owned()
  })
}

//  this function is a helper function of search_professor_id (it will be called within the function body)
pub async fn get_school_id(school_name : &str) -> String {
  let schools = search_school(school_name).await.unwrap();
  let mut school_id : &str = "";
  if let Some(school) = schools.first() {
      school_id = &school.node.id;
    }
    println!("School id retrieved successfully : {school_id:?}");
    school_id.to_owned()
}
 
/// graphql queries should be json based strings
const TEACHER_BODY_QUERY: &str = r#"query TeacherSearchResultsPageQuery(
  $query: TeacherSearchQuery!
  $schoolID: ID
  $includeSchoolFilter: Boolean!
) {
  search: newSearch {
    ...TeacherSearchPagination_search_1ZLmLD
  }
  school: node(id: $schoolID) @include(if: $includeSchoolFilter) {
    __typename
    ... on School {
      name
    }
    id
  }
}

fragment TeacherSearchPagination_search_1ZLmLD on newSearch {
  teachers(query: $query, first: 8, after: "") {
    didFallback
    edges {
      cursor
      node {
        ...TeacherCard_teacher
        id
        __typename
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
    resultCount
    filters {
      field
      options {
        value
        id
      }
    }
  }
}

fragment TeacherCard_teacher on Teacher {
  id
  legacyId
  avgRating
  numRatings
  ...CardFeedback_teacher
  ...CardSchool_teacher
  ...CardName_teacher
  ...TeacherBookmark_teacher
}

fragment CardFeedback_teacher on Teacher {
  wouldTakeAgainPercent
  avgDifficulty
}

fragment CardSchool_teacher on Teacher {
  department
  school {
    name
    id
  }
}

fragment CardName_teacher on Teacher {
  firstName
  lastName
}

fragment TeacherBookmark_teacher on Teacher {
  id
  isSaved
}"#;

const TEACHER_LIST_QUERY : &str = r#"query TeacherSearchResultsPageQuery(
        $query: TeacherSearchQuery!
        $schoolID: ID
        $includeSchoolFilter: Boolean!
    ) {
        search: newSearch {
            teachers(query: $query, first: 1000, after: "") {
                edges {
                    node {
                        id
                        legacyId
                        firstName
                        lastName
                        department
                        avgRating
                        numRatings
                        wouldTakeAgainPercent
                        avgDifficulty
                        school {
                            name
                            id
                        }
                    }
                }
                pageInfo {
                    hasNextPage
                    endCursor
                }
                resultCount
            }
        }
        school: node(id: $schoolID) @include(if: $includeSchoolFilter) {
            __typename
            ... on School {
                name
            }
            id
        }
    }"#;

const SCHOOL_BODY_QUERY: &str = r#"query NewSearchSchoolsQuery(
  $query: SchoolSearchQuery!
) {
  newSearch {
    schools(query: $query) {
      edges {
        cursor
        node {
          id
          legacyId
          name
          city
          state
          departments {
            id
            name
          }
          numRatings
          avgRatingRounded
          summary {
            campusCondition
            campusLocation
            careerOpportunities
            clubAndEventActivities
            foodQuality
            internetSpeed
            libraryCondition
            schoolReputation
            schoolSafety
            schoolSatisfaction
            socialActivities
          }
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
  }
}"#;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeacherSearch {
    pub cursor: String,
    pub node: TeacherNode,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct ProfessorRating {
    #[serde(rename="avgRating")]
    pub avg_rating: f64,

    #[serde(rename="avgDifficulty")]
    pub avg_difficulty: f64,

    #[serde(rename="wouldTakeAgainPercent")]
    pub would_take_again_percent: f64,

    #[serde(rename="numRatings")]
    pub num_ratings: i32,

    #[serde(rename="formattedName")]
    pub formatted_name: String,
    pub department: String,

    #[serde(rename="name")]
    pub college_name : String,    // newly added
    pub link: String,
}

/// HeaderValue::from_static : convert a static string to a HeaderValue
/// This function will not perform any copying, becasue the goal is to ensure that the string is checked to ensure that no invalid characters are present and that only visible ASCII characters are permitted.
fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Authorization", HeaderValue::from_static("Basic dGVzdDp0ZXN0"));
    headers.insert("Sec-GPC", HeaderValue::from_static("1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Priority", HeaderValue::from_static("u=4"));
    headers
}

pub async fn search_professors_at_school_id(
    professor_name: &str,
    school_id: &str,
) -> Result<Vec<TeacherSearch>> {
    let client = reqwest::Client::new();
    
    let variables = serde_json::json!({
        "query": {
            "text": professor_name,
            "schoolID": school_id,
            "fallback": true,
            "departmentID": null
        },
        "schoolID": school_id,
        "includeSchoolFilter": true
    });
    
    let body = serde_json::json!({
        "query": TEACHER_BODY_QUERY,
        "variables": variables
    });

    let response = client
        .post(API_LINK)
        .headers(get_headers())
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Network response from RMP not OK"));
    }

    let data: serde_json::Value = response.json().await?;
    let edges = data["data"]["search"]["teachers"]["edges"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse teacher search results"))?;
    let results: Vec<TeacherSearch> = serde_json::from_value(serde_json::Value::Array(edges.to_vec()))?;
    Ok(results)
}

pub async fn get_professor_rating_at_school_id(
    professor_name: &str,
    school_id: &str,
) -> Result<ProfessorRating> {
    let search_results = search_professors_at_school_id(professor_name, school_id).await?;
    if search_results.is_empty() {
        return Ok(ProfessorRating {
            avg_rating: -1.0,
            avg_difficulty: -1.0,
            would_take_again_percent: -1.0,
            num_ratings: 0,
            formatted_name: professor_name.to_string(),
            department: String::new(),
            college_name : String::new(),
            link: String::new(),
        });
    }

    let professor_result = &search_results[0];
    Ok(ProfessorRating {
        avg_rating: professor_result.node.avg_rating,
        avg_difficulty: professor_result.node.avg_difficulty,
        would_take_again_percent: professor_result.node.would_take_again_percent,
        num_ratings: professor_result.node.num_ratings,
        formatted_name: format!(
            "{} {}",
            professor_result.node.first_name, professor_result.node.last_name
        ),
        college_name : professor_result.node.school.name.clone(),
        department: professor_result.node.department.clone(),
        link: format!(
            "https://www.ratemyprofessors.com/professor/{}",
            professor_result.node.legacy_id
        ),
    })
}

// working example showing how to retrieve a college ID given a college name
// working example showing how to retrieve a teacher ID given a teacher and college name
// copied within bin/execute_features.rs
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolSummary {
    #[serde(rename = "campusCondition")]
    pub campus_condition: Option<f64>,
    #[serde(rename = "campusLocation")]
    pub campus_location: Option<f64>,
    #[serde(rename = "careerOpportunities")]
    pub career_opportunities: Option<f64>,
    #[serde(rename = "clubAndEventActivities")]
    pub club_and_event_activities: Option<f64>,
    #[serde(rename = "foodQuality")]
    pub food_quality: Option<f64>,
    #[serde(rename = "internetSpeed")]
    pub internet_speed: Option<f64>,
    #[serde(rename = "libraryCondition")]
    pub library_condition: Option<f64>,
    #[serde(rename = "schoolReputation")]
    pub school_reputation: Option<f64>,
    #[serde(rename = "schoolSafety")]
    pub school_safety: Option<f64>,
    #[serde(rename = "schoolSatisfaction")]
    pub school_satisfaction: Option<f64>,
    #[serde(rename = "socialActivities")]
    pub social_activities: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolNode {
    #[serde(rename = "avgRatingRounded")]
    pub avg_rating_rounded: f64,
    pub city: String,
    pub departments: Vec<Department>,
    pub id: String,
    #[serde(rename = "legacyId")]
    pub legacy_id: i64,
    pub name: String,
    #[serde(rename = "numRatings")]
    pub num_ratings: i32,
    pub state: String,
    pub summary: SchoolSummary,     // nested struct
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchoolSearch {
    pub cursor: String,
    pub node: SchoolNode,
}

pub async fn search_school(school_name: &str) -> Result<Vec<SchoolSearch>> {
    // initialize the reqwest client that will be used to make the API calls
    let client = reqwest::Client::new();
    let variables = serde_json::json!({
        "query": {
            "text": school_name
        }
    });


    /**
     * for "variables", it would be the same as writting
     * "variables" : "query" : { "text" : school_name }
     */
    let body = serde_json::json!({
        "query": SCHOOL_BODY_QUERY,
        "variables": variables
    });

    // make the POST request to the external graphql endpoint
    // pass in the header map returned from get_headers() function
    // .json() used to indicate the json body, which references the graphql query
    // .send() indicates confirmation to send the request and store the response within response variable
    let response = client
        .post(API_LINK)
        .headers(get_headers())
        .json(&body)
        .send()
        .await?;

    // check and verify if the .status() of the response is successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Network response from RMP not OK"));
    }

    let data: serde_json::Value = response.json().await?;
    let edges = data["data"]["newSearch"]["schools"]["edges"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse school search results"))?;

    // converts string from reference string string only
    let school_id : String = serde_json::from_str(&edges[0]["cursor"].clone().to_string()).unwrap();
    let results: Vec<SchoolSearch> = serde_json::from_value(serde_json::Value::Array(edges.to_vec()))?;
    let results_json = serde_json::to_string(&results);

    if results_json.is_err() {
      println!("Error, failed to serailize data : {}", results_json.unwrap_err());
      std::process::exit(1);
    }
    let result_json_string = results_json.unwrap();

    // "unpack" the 2 values
    let (all_search_result_file, file_path) = create_file("all_search_result.json").await;
    let (school_name_file, file_path) = create_file(&(school_name.clone().to_owned() + ".json")).await;
    save_data_to_file(all_search_result_file, &result_json_string).await;

    for (index, data) in results.clone().into_iter().enumerate() {
      if data.node.name == school_name {
        let data_json = serde_json::to_string(&data);

        if data_json.is_err() {
          println!("Failed to serialize data : {:?}", data_json.unwrap_err());
          std::process::exit(1);
        }
        // upon successful conversion, unwrap() the data
        let data_json_string = data_json.unwrap();
        save_data_to_file(school_name_file, &data_json_string).await;
        break;
      }
    }
    // println
    Ok(results)
}

// helper function to retrieve the type
fn print_type_of<T>(_ : &T) {
  println!("{}", std::any::type_name::<T>());
}

// function to save the content
// returns nothing, inplace modification
pub async fn save_data_to_file(mut file : fs::File, data : &str) {
  file.write_all(data.as_bytes()).expect("failed to write json data to file")
}

// function returns a tuple of values -> the file and the path to the file
pub async fn create_file(fileName : &str) -> (fs::File, PathBuf) {
  let mut file = fs::File::create(fileName).unwrap();
  let filePath = file.path().unwrap();    // Ok("/path/to/file") -> "/path/to/file"
  (file, filePath)
}

// define the struct that will handle retrieving the list of professors given a specific college
// need to call upon search_school to retrieve the list
// struct shares a lot of similarities with some of the other existing structs
// automatically enabled to handle null data since large quantity of data is being gathered here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessorList {
  pub id : Option<String>,
  pub legacy_id : Option<String>,
  pub first_name : Option<String>,
  pub last_name : Option<String>,
  pub department : Option<String>,
  pub avg_rating : Option<f64>,
  pub num_rating : Option<i32>,
  pub avg_difficulty : Option<f64>
}

/// retrieves a list of professors for a specific college based on the college id
/// original result type : Result<Vec<ProfessorList>>
pub async fn get_professor_list_by_school(college_id : &str) -> Result<Vec<ProfessorList>> {
  let mut professor_list : Vec<ProfessorList> = Vec::new();
  let client = reqwest::Client::new();
  let payload = serde_json::json!({
    "query" : TEACHER_LIST_QUERY,
    "variables" : {
      "query" : {
        "text" : "",    // empty : ensures all professor list is retrieved
        "schoolID" : college_id,
        "fallback" : true,
        "departmentID" : null,
      },
      "schoolID" : college_id,
      "includeSchoolFilter" : true
    },
  });
  let response = client.post(API_LINK).headers(get_headers()).json(&payload).send().await?;
  if !response.status().is_success() {
    return Err(anyhow::anyhow!("Network response from RMP not OK"));
  }

  let mut professor_list_raw : serde_json::Value = response.json().await?;
  
  // break down the data to the edges array so we can itereate over it
  let professor_list_edges = professor_list_raw["data"]["search"]["teachers"]["edges"].clone();

  // retrieve the length and iterate over the range to construct the vector that will store the data
  let professor_list_edges_length = get_json_length(&professor_list_edges);
  for curr_index in 0..professor_list_edges_length {

    let unique_id : Option<String> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["id"].to_string())?);

    let legacy_id : Option<String> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["id"].to_string())?);

    let first_name : Option<String> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["firstName"].to_string())?);

    let last_name : Option<String> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["lastName"].to_string())?);

    let department : Option<String> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["department"].to_string())?);

    let avg_rating : Option<f64> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["avgRating"].to_string())?);

    let num_rating : Option<i32> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["numRatings"].to_string())?);

    let avg_difficulty : Option<f64> = Some(serde_json::from_str(&professor_list_edges[curr_index]["node"]["avgDifficulty"].to_string())?);

    // TODO : construct list
    let professor_list_instance = ProfessorList {
      id : unique_id,
      legacy_id : legacy_id,
      first_name : first_name,
      last_name : last_name,
      department : department,
      avg_rating : avg_rating,
      num_rating : num_rating,
      avg_difficulty : avg_difficulty,
    };
    professor_list.push(professor_list_instance);
  }

  Ok(professor_list)
}