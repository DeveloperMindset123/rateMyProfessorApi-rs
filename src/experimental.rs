use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::{fs,io::Write};
#[derive(Debug, Serialize, Deserialize)]
pub struct School {
    pub id: String,   
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherNode {
    pub __typename: String,   // unused variable

    #[serde(rename="avgDifficulty")]            // formatting, matches the current value to what the returned object type would be using this rust attribute
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


#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherSearch {
    pub cursor: String,
    pub node: TeacherNode,
}

#[derive(Debug, Serialize, Deserialize)]
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
    println!("{:?}", data);

    // error occuring here
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
    println!("search result for professor is : {:?}", search_results);
    if search_results.is_empty() {
        return Ok(ProfessorRating {
            avg_rating: -1.0,
            avg_difficulty: -1.0,
            would_take_again_percent: -1.0,
            num_ratings: 0,
            formatted_name: professor_name.to_string(),
            department: String::new(),
            link: String::new(),
        });
    }

    let professor_result = &search_results[0];
    println!("resulting professor result : {:?}", professor_result);
    Ok(ProfessorRating {
        avg_rating: professor_result.node.avg_rating,
        avg_difficulty: professor_result.node.avg_difficulty,
        would_take_again_percent: professor_result.node.would_take_again_percent,
        num_ratings: professor_result.node.num_ratings,
        formatted_name: format!(
            "{} {}",
            professor_result.node.first_name, professor_result.node.last_name
        ),
        department: professor_result.node.department.clone(),
        link: format!(
            "https://www.ratemyprofessors.com/professor/{}",
            professor_result.node.legacy_id
        ),
    })
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     // let rating = get_professor_rating_at_school_id("Neil Henry", "YXJyYXljb25uZWN0aW9uOjA=").await?;
//     let school = search_school("University of California Berkley").await;
//     Ok(())
// }

// TODO : implement this
// async fn save_to_file(returned_json) -> Result<()> {

// }
#[tokio::main]
async fn main() -> Result<()> {
    // First search for a school
    let schools = search_school("University of California").await?;
    
    if let Some(school) = schools.first() {
        println!("Found school: {} in {}, {}", 
            school.node.name, 
            school.node.city, 
            school.node.state
        );
        
        // Then search for professors at that school
        let school_id = &school.node.id;
        // println!("{:?}", school_id);
        let professors = search_professors_at_school_id("Jean Frechet", &school.node.id).await?;
        println!("Professors : {:?}", professors);
        for professor in professors {
            println!("Found professor: {} {} in {}",
                professor.node.first_name,
                professor.node.last_name,
                professor.node.department
            );
            
            // Get detailed rating
            let rating = get_professor_rating_at_school_id(
                &format!("{} {}", professor.node.first_name, professor.node.last_name),
                &school.node.id
            ).await?;
            
            println!("Rating: {}/5.0, Average Difficulty : {}/5.0", 
                rating.avg_rating,
                rating.avg_difficulty
            );
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolSearch {
    pub cursor: String,
    pub node: SchoolNode,
}

pub async fn search_school(school_name: &str) -> Result<Vec<SchoolSearch>> {
    let client = reqwest::Client::new();
    
    let variables = serde_json::json!({
        "query": {
            "text": school_name
        }
    });

    let body = serde_json::json!({
        "query": SCHOOL_BODY_QUERY,
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
    // println!("{:?}", data);
    // println!("{:?}", data["data"]["newSearch"]["schools"]);
    let edges = data["data"]["newSearch"]["schools"]["edges"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse school search results"))?;

    // converts string from reference string string only
    let school_id : String = serde_json::from_str(&edges[0]["cursor"].clone().to_string()).unwrap();
    // println!("{:?}", school_id);
    let results: Vec<SchoolSearch> = serde_json::from_value(serde_json::Value::Array(edges.to_vec()))?;

    // println!("Resulting output is : {:?}", results);
    let results_json = serde_json::to_string(&results);

    if results_json.is_err() {
      println!("Error, failed to serailize data : {}", results_json.unwrap_err());
      std::process::exit(1);
    }
    // otherwise, if serialziation is successful
    let result_json_string = results_json.unwrap();
    println!("Serialized json string data : {}", result_json_string);
    let mut f = fs::File::create("test.json").expect("failed to create file");
    f.write_all(result_json_string.as_bytes()).expect("failed to write json data to file");
    Ok(results)
}