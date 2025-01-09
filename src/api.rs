// use hyper::header::{Connection,UserAgent};
use actix_web::{get, web, Responder, Result, App, HttpServer, Error};    
use actix_rt::System;
use serde_json::{Result as AnotherResult, Value, Map};
use serde_json::json;
use awc::Client;
use std::fs;
use std::io::Write;
// use awc::body::None;
// list of relevant api calls as part of reverse engineering process
// https://d3hb14vkzrxvla.cloudfront.net/v1/f9787baf-9436-454f-9561-731c100141f7 : this is the url for loading the api --> this will lead to city college of new york


// // TODO : create methods to gather a list of courses sorted based on the catalog.
// // base API Link
const API_LINK : &str = "https://www.ratemyprofessors.com/graphql";

// pub struct 

// pub struct HeaderInfo {
//   UserAgent : String
// }

// const CUSTOM_USER_AGENT : &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"; 
// "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0"
const HEADERS : &str = r#"
{
  "User-Agent" : "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
  "Accept": "*/*",
  "Accept-Language": "en-US,en;q=0.5",
  "Content-Type": "application/json",
  "Authorization": "Basic dGVzdDp0ZXN0",
  "Sec-GPC": "1",
  "Sec-Fetch-Dest": "empty",
  "Sec-Fetch-Mode": "cors",
  "Sec-Fetch-Site": "same-origin",
  "Priority" : "u=4"
}"#;
// const HEADERS = {
//   "User-Agent":
//     "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:129.0) Gecko/20100101 Firefox/129.0",
//   Accept: "*/*",
//   "Accept-Language": "en-US,en;q=0.5",
//   "Content-Type": "application/json",
//   Authorization: "Basic dGVzdDp0ZXN0",
//   "Sec-GPC": "1",
//   "Sec-Fetch-Dest": "empty",
//   "Sec-Fetch-Mode": "cors",
//   "Sec-Fetch-Site": "same-origin",
//   Priority: "u=4",
// };


// const TEACHER_BODY_QUERY: &str = r#"
// query TeacherSearchResultsPageQuery(
//   $query: TeacherSearchQuery!
//   $schoolID: ID
//   $includeSchoolFilter: Boolean!
// ) {
//   search: newSearch {
//     ...TeacherSearchPagination_search_1ZLmLD
//   }
//   school: node(id: $schoolID) @include(if: $includeSchoolFilter) {
//     __typename
//     ... on School {
//       name
//     }
//     id
//   }
// }

// fragment TeacherSearchPagination_search_1ZLmLD on newSearch {
//   teachers(query: $query, first: 8, after: "") {
//     didFallback
//     edges {
//       cursor
//       node {
//         ...TeacherCard_teacher
//         id
//         __typename
//       }
//     }
//     pageInfo {
//       hasNextPage
//       endCursor
//     }
//     resultCount
//     filters {
//       field
//       options {
//         value
//         id
//       }
//     }
//   }
// }

// fragment TeacherCard_teacher on Teacher {
//   id
//   legacyId
//   avgRating
//   numRatings
//   ...CardFeedback_teacher
//   ...CardSchool_teacher
//   ...CardName_teacher
//   ...TeacherBookmark_teacher
// }

// fragment CardFeedback_teacher on Teacher {
//   wouldTakeAgainPercent
//   avgDifficulty
// }

// fragment CardSchool_teacher on Teacher {
//   department
//   school {
//     name
//     id
//   }
// }

// fragment CardName_teacher on Teacher {
//   firstName
//   lastName
// }

// fragment TeacherBookmark_teacher on Teacher {
//   id
//   isSaved
// }
// "#;

// const TEACHER_BODY_QUERY : &str ='"query TeacherSearchResultsPageQuery(\\n  $query: TeacherSearchQuery!\\n  $schoolID: ID\\n  $includeSchoolFilter: Boolean!\\n) {\\n  search: newSearch {\\n    ...TeacherSearchPagination_search_1ZLmLD\\n  }\\n  school: node(id: $schoolID) @include(if: $includeSchoolFilter) {\\n    __typename\\n    ... on School {\\n      name\\n    }\\n    id\\n  }\\n}\\n\\nfragment TeacherSearchPagination_search_1ZLmLD on newSearch {\\n  teachers(query: $query, first: 8, after: \\"\\") {\\n    didFallback\\n    edges {\\n      cursor\\n      node {\\n        ...TeacherCard_teacher\\n        id\\n        __typename\\n      }\\n    }\\n    pageInfo {\\n      hasNextPage\\n      endCursor\\n    }\\n    resultCount\\n    filters {\\n      field\\n      options {\\n        value\\n        id\\n      }\\n    }\\n  }\\n}\\n\\nfragment TeacherCard_teacher on Teacher {\\n  id\\n  legacyId\\n  avgRating\\n  numRatings\\n  ...CardFeedback_teacher\\n  ...CardSchool_teacher\\n  ...CardName_teacher\\n  ...TeacherBookmark_teacher\\n}\\n\\nfragment CardFeedback_teacher on Teacher {\\n  wouldTakeAgainPercent\\n  avgDifficulty\\n}\\n\\nfragment CardSchool_teacher on Teacher {\\n  department\\n  school {\\n    name\\n    id\\n  }\\n}\\n\\nfragment CardName_teacher on Teacher {\\n  firstName\\n  lastName\\n}\\n\\nfragment TeacherBookmark_teacher on Teacher {\\n  id\\n  isSaved\\n}\\n"';

const TEACHER_BODY_QUERY : &str ="\"query TeacherSearchResultsPageQuery(\\n  $query: TeacherSearchQuery!\\n  $schoolID: ID\\n  $includeSchoolFilter: Boolean!\\n) {\\n  search: newSearch {\\n    ...TeacherSearchPagination_search_1ZLmLD\\n  }\\n  school: node(id: $schoolID) @include(if: $includeSchoolFilter) {\\n    __typename\\n    ... on School {\\n      name\\n    }\\n    id\\n  }\\n}\\n\\nfragment TeacherSearchPagination_search_1ZLmLD on newSearch {\\n  teachers(query: $query, first: 8, after: \\\"\\\") {\\n    didFallback\\n    edges {\\n      cursor\\n      node {\\n        ...TeacherCard_teacher\\n        id\\n        __typename\\n      }\\n    }\\n    pageInfo {\\n      hasNextPage\\n      endCursor\\n    }\\n    resultCount\\n    filters {\\n      field\\n      options {\\n        value\\n        id\\n      }\\n    }\\n  }\\n}\\n\\nfragment TeacherCard_teacher on Teacher {\\n  id\\n  legacyId\\n  avgRating\\n  numRatings\\n  ...CardFeedback_teacher\\n  ...CardSchool_teacher\\n  ...CardName_teacher\\n  ...TeacherBookmark_teacher\\n}\\n\\nfragment CardFeedback_teacher on Teacher {\\n  wouldTakeAgainPercent\\n  avgDifficulty\\n}\\n\\nfragment CardSchool_teacher on Teacher {\\n  department\\n  school {\\n    name\\n    id\\n  }\\n}\\n\\nfragment CardName_teacher on Teacher {\\n  firstName\\n  lastName\\n}\\n\\nfragment TeacherBookmark_teacher on Teacher {\\n  id\\n  isSaved\\n}\\n\"";

const SCHOOL_BODY_QUERY : &str = "\"query NewSearchSchoolsQuery(\\n  $query: SchoolSearchQuery!\\n) {\\n  newSearch {\\n    schools(query: $query) {\\n      edges {\\n        cursor\\n        node {\\n          id\\n          legacyId\\n          name\\n          city\\n          state\\n          departments {\\n            id\\n            name\\n          }\\n          numRatings\\n          avgRatingRounded\\n          summary {\\n            campusCondition\\n            campusLocation\\n            careerOpportunities\\n            clubAndEventActivities\\n            foodQuality\\n            internetSpeed\\n            libraryCondition\\n            schoolReputation\\n            schoolSafety\\n            schoolSatisfaction\\n            socialActivities\\n          }\\n        }\\n      }\\n      pageInfo {\\n        hasNextPage\\n        endCursor\\n      }\\n    }\\n  }\\n}\\n\"";

// // another potential GET method that may be useful:
// // https://app.coursedog.com/api/v1/cty01/pdf-reports/catalogs/tyrc1I8cy2QhVy5W5L2I

// // Below are example links related to CUNY City College courses
// https://app.coursedog.com/api/v1/cm/cty01/courses/search/$filters?courseGroupIds=0584301&effectiveDatesRange=2024-08-28%2C2024-08-28&formatDependents=false&includeRelatedData=true&includeCrosslisted=false&includeCourseEquivalencies=true&includeMappedDocumentItems=true&includePending=false&returnResultsWithTotalCount=false&doNotDisplayAllMappedRevisionsAsDependencies=true&columns=departments%2CcourseTypicallyOffered%2Ccareer%2Ccredits%2Ccomponents%2Ctopics%2CcatalogAttributes%2Cdescription%2CrequirementGroup%2CcourseSchedule%2CcustomFields.ZK6fC%2ClongName%2Cinstitution%2Cconsent%2CcustomFields.cuPathwaysAttribute%2CsubjectCode%2CcourseNumber%2CcustomFields.cuLibartsFlag%2Ccode%2Cname%2Ccollege%2Cstatus%2CinstitutionId%2CrawCourseId%2CcrseOfferNbr%2CcustomFields.catalogAttributes%2CcustomFields.rawCourseId : this would be the relevant link for searching up a particular course


// // 
// https://prod-eks-catalog.s3.us-east-2.amazonaws.com/4b843c1/39.7762a16c0fba4205a211.js : this is another link to make sense of, but not entirely sure what purpose it serves.

// // 
// https://app.coursedog.com/api/v1/cm/cty01/courses/search/%24filters?catalogId=tyrc1I8cy2QhVy5W5L2I&skip=0&limit=20&orderBy=catalogDisplayName%2CtranscriptDescription%2ClongName%2Cname&formatDependents=false&effectiveDatesRange=2024-08-28%2C2024-08-28&columns=displayName%2Cdepartment%2Cname%2CcourseNumber%2CsubjectCode%2Ccode%2CcourseGroupId%2Ccredits.creditHours%2ClongName%2Ccareer%2Ccomponents%2CcustomFields.catalogRequirementDesignation%2CcustomFields.catalogAttributes : Link to filter courses by architecture department, a single GET request using this will suffice


// // 
// https://app.coursedog.com/api/v1/cm/cty01/courses/search/%24filters?catalogId=tyrc1I8cy2QhVy5W5L2I&skip=0&limit=20&orderBy=catalogDisplayName%2CtranscriptDescription%2ClongName%2Cname&formatDependents=false&effectiveDatesRange=2024-08-28%2C2024-08-28&columns=displayName%2Cdepartment%2Cname%2CcourseNumber%2CsubjectCode%2Ccode%2CcourseGroupId%2Ccredits.creditHours%2ClongName%2Ccareer%2Ccomponents%2CcustomFields.catalogRequirementDesignation%2CcustomFields.catalogAttributes : courses for biology

// // https://app.coursedog.com/api/v1/cty01/requirementGroups/018401?returnFields=code,catalogDisplayName,displayName,descriptionLong : another relevant endpoint that will be useful


pub fn test_main() -> Result<()> {
//   let custom_agent : &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
//   let header : &str = r#"
// {
//   "User-Agent" : $custom_agent",
//   "Accept": "*/*",
//   "Accept-Language": "en-US,en;q=0.5",
//   "Content-Type": "application/json",
//   "Authorization": "Basic dGVzdDp0ZXN0",
//   "Sec-GPC": "1",
//   "Sec-Fetch-Dest": "empty",
//   "Sec-Fetch-Mode": "cors",
//   "Sec-Fetch-Site": "same-origin",
//   "Priority" : "u=4"
// }"#;
  // let header_data : Value = serde_json::from_str(header)?;
  // println!("{:?}",&header);
  let val : i32  = 20;
  // let data = val;

  // this is to understand how string formatting works
  // let sample_test_data : Value = json!({
  //   "sample_val" : data
  // });

  // let sample_json_string = r#"{
  //     "some_random_data" : $data
  // }"#;
  // let sample_test_json : Value = serde_json::from_str(sample_json_string)?;


  // simple example showing how to create a json object containing variables using json! macro
  let json_data = json!({
    "some_key" : val
  });

  let teacher_body_query_json : Value = serde_json::from_str(TEACHER_BODY_QUERY)?;
  println!("{:?}", teacher_body_query_json);
  Ok(())
}

// pub async fn searchSchools(
//   schoolName : String
// ) -> 


// simple example for specifying return type in actix
// this is just here for testing
#[get("/{val}")]
async fn index (val : web::Path<String>) -> Result<impl Responder> {
  let json_data = json!({
    "input_data" : val.to_owned(),
  });

  Ok(web::Json(json_data))
}

async fn searchSchool(
  schoolName : String,
) -> std::io::Result<()> {
  let json_body : serde_json::Value = json!({
    "query" : SCHOOL_BODY_QUERY,
    "variables" : {
      "query" : {
        "text" : schoolName.to_owned()
      }
    }
  });

  let mut client = Client::default();
    // experimentation by inserting headers 1-by-1
    let res = client.post("https://www.ratemyprofessors.com/graphql")
        .insert_header(("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"))
        .insert_header(("Accept", "*/*"))
        .insert_header(("Accept-Language", "en-US,en;q=0.5"))
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", "Basic dGVzdDp0ZXN0"))
        .insert_header(("Sec-GPC","1"))
        .insert_header(("Sec-Fetch-Dest", "empty"))
        .insert_header(("Sec-Fetch-Mode","cors"))
        .insert_header(("Sec-Fetch-Site","same-origin"))
        .insert_header(("Priority","u=4"))
        .send_json(&json_body)
        .await;

      println!("{:?}", res);
      Ok(())

}
async fn searchProfessorViaSchoolID(professorName : String, schoolID : String) {
    let json_body : serde_json::Value = json!({
          "query" : TEACHER_BODY_QUERY.to_owned(),
          "variables" : {
            "query" : {
              "text" : professorName,
              "schoolID" : schoolID,
              "fallback" : true,
              // "departmentID" : None::new(),
            },
            "schoolId" : schoolID,
            "includeSchoolFilter" : true
          }}
        );
    let mut client = Client::default();
    // experimentation by inserting headers 1-by-1
    let res = client.post("https://www.ratemyprofessors.com/graphql")
        .insert_header(("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"))
        .insert_header(("Accept", "*/*"))
        .insert_header(("Accept-Language", "en-US,en;q=0.5"))
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", "Basic dGVzdDp0ZXN0"))
        .insert_header(("Sec-GPC","1"))
        .insert_header(("Sec-Fetch-Dest", "empty"))
        .insert_header(("Sec-Fetch-Mode","cors"))
        .insert_header(("Sec-Fetch-Site","same-origin"))
        .insert_header(("Priority","u=4"))
        .send_json(&json_body)
        .await;
        // .map_err(|_| ())
        // .and_then(|response| {
        //   println!("API Response is : {:?}", response);
        //   Ok(())
        // });
    // let response = res.and_then(|mut response| {
    //   println!("Response : {:?}", response);
    //   Ok(web::Json(response.body()))
    // });

    // let mut data_file = fs::write("data.txt", res.body()).expect("Unable to write to file");
    println!("{:#?}", res);
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // test_api_calls("douglas troeger".to_string(), "W-dLO_mXSL-20241225".to_string()).await;

  let res = searchSchool("University of California Berkeley".to_string()).await;
  println!("{:?}",res);
  HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}