// store graph ql queries here

/// query to retrieve informatin about teacher given a college ID
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


/// query to retrieve information about a school given a school name
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


// The query string properly escaped for Rust
const query : &str = r#"
{
  "query": "query TeacherRatingsPageQuery($id: ID!) { node(id: $id) { __typename ... on Teacher { id legacyId firstName lastName department school { legacyId name city state country id } lockStatus ...StickyHeaderContent_teacher ...RatingDistributionWrapper_teacher ...TeacherInfo_teacher ...SimilarProfessors_teacher ...TeacherRatingTabs_teacher } id } } fragment StickyHeaderContent_teacher on Teacher { ...HeaderDescription_teacher ...HeaderRateButton_teacher } fragment RatingDistributionWrapper_teacher on Teacher { ...NoRatingsArea_teacher ratingsDistribution { total ...RatingDistributionChart_ratingsDistribution } } fragment TeacherInfo_teacher on Teacher { id lastName numRatings ...RatingValue_teacher ...NameTitle_teacher ...TeacherTags_teacher ...NameLink_teacher ...TeacherFeedback_teacher ...RateTeacherLink_teacher ...CompareProfessorLink_teacher } fragment SimilarProfessors_teacher on Teacher { department relatedTeachers { legacyId ...SimilarProfessorListItem_teacher id } } fragment TeacherRatingTabs_teacher on Teacher { numRatings courseCodes { courseName courseCount } ...RatingsList_teacher ...RatingsFilter_teacher } fragment RatingsList_teacher on Teacher { id legacyId lastName numRatings school { id legacyId name city state avgRating numRatings } ...Rating_teacher ...NoRatingsArea_teacher ratings(first: 20) { edges { cursor node { ...Rating_rating id __typename } } pageInfo { hasNextPage endCursor } } } fragment RatingsFilter_teacher on Teacher { courseCodes { courseCount courseName } } fragment Rating_teacher on Teacher { ...RatingFooter_teacher ...RatingSuperHeader_teacher ...ProfessorNoteSection_teacher } fragment NoRatingsArea_teacher on Teacher { lastName ...RateTeacherLink_teacher } fragment Rating_rating on Rating { comment flagStatus createdByUser teacherNote { id } ...RatingHeader_rating ...RatingSuperHeader_rating ...RatingValues_rating ...CourseMeta_rating ...RatingTags_rating ...RatingFooter_rating ...ProfessorNoteSection_rating } fragment RatingHeader_rating on Rating { legacyId date class helpfulRating clarityRating isForOnlineClass } fragment RatingSuperHeader_rating on Rating { legacyId } fragment RatingValues_rating on Rating { helpfulRating clarityRating difficultyRating } fragment CourseMeta_rating on Rating { attendanceMandatory wouldTakeAgain grade textbookUse isForOnlineClass isForCredit } fragment RatingTags_rating on Rating { ratingTags } fragment RatingFooter_rating on Rating { id comment adminReviewedAt flagStatus legacyId thumbsUpTotal thumbsDownTotal thumbs { thumbsUp thumbsDown computerId id } teacherNote { id } ...Thumbs_rating } fragment ProfessorNoteSection_rating on Rating { teacherNote { ...ProfessorNote_note id } ...ProfessorNoteEditor_rating } fragment ProfessorNote_note on TeacherNotes { comment ...ProfessorNoteHeader_note ...ProfessorNoteFooter_note } fragment ProfessorNoteEditor_rating on Rating { id legacyId class teacherNote { id teacherId comment } } fragment ProfessorNoteHeader_note on TeacherNotes { createdAt updatedAt } fragment ProfessorNoteFooter_note on TeacherNotes { legacyId flagStatus } fragment Thumbs_rating on Rating { id comment adminReviewedAt flagStatus legacyId thumbsUpTotal thumbsDownTotal thumbs { computerId thumbsUp thumbsDown id } teacherNote { id } } fragment RateTeacherLink_teacher on Teacher { legacyId numRatings lockStatus } fragment RatingFooter_teacher on Teacher { id legacyId lockStatus isProfCurrentUser ...Thumbs_teacher } fragment RatingSuperHeader_teacher on Teacher { firstName lastName legacyId school { name id } } fragment ProfessorNoteSection_teacher on Teacher { ...ProfessorNote_teacher ...ProfessorNoteEditor_teacher } fragment ProfessorNote_teacher on Teacher { ...ProfessorNoteHeader_teacher ...ProfessorNoteFooter_teacher } fragment ProfessorNoteEditor_teacher on Teacher { id } fragment ProfessorNoteHeader_teacher on Teacher { lastName } fragment ProfessorNoteFooter_teacher on Teacher { legacyId isProfCurrentUser } fragment Thumbs_teacher on Teacher { id legacyId lockStatus isProfCurrentUser } fragment SimilarProfessorListItem_teacher on RelatedTeacher { legacyId firstName lastName avgRating } fragment RatingValue_teacher on Teacher { avgRating numRatings ...NumRatingsLink_teacher } fragment NameTitle_teacher on Teacher { id firstName lastName department school { legacyId name id } ...TeacherDepartment_teacher ...TeacherBookmark_teacher } fragment TeacherTags_teacher on Teacher { lastName teacherRatingTags { legacyId tagCount tagName id } } fragment NameLink_teacher on Teacher { isProfCurrentUser id legacyId firstName lastName school { name id } } fragment TeacherFeedback_teacher on Teacher { numRatings avgDifficulty wouldTakeAgainPercent } fragment CompareProfessorLink_teacher on Teacher { legacyId } fragment TeacherDepartment_teacher on Teacher { department departmentId school { legacyId name id } } fragment TeacherBookmark_teacher on Teacher { id isSaved } fragment NumRatingsLink_teacher on Teacher { numRatings ...RateTeacherLink_teacher } fragment RatingDistributionChart_ratingsDistribution on ratingsDistribution { r1 r2 r3 r4 r5 } fragment HeaderDescription_teacher on Teacher { id legacyId firstName lastName department school { legacyId name city state id } ...TeacherTitles_teacher ...TeacherBookmark_teacher ...RateTeacherLink_teacher ...CompareProfessorLink_teacher } fragment HeaderRateButton_teacher on Teacher { ...RateTeacherLink_teacher ...CompareProfessorLink_teacher } fragment TeacherTitles_teacher on Teacher { department school { legacyId name id } }",
  "variables": {
    "id": "your_teacher_id_here"
  }
}"#;


// const SAMPLE_QUERY : &str = r#"query TeacherRatingsPageQuery(\n  $id: ID!\n) {\n  node(id: $id) {\n    __typename\n    ... on Teacher {\n      id\n      legacyId\n      firstName\n      lastName\n      department\n      school {\n        legacyId\n        name\n        city\n        state\n        country\n        id\n      }\n      lockStatus\n      ...StickyHeaderContent_teacher\n      ...RatingDistributionWrapper_teacher\n      ...TeacherInfo_teacher\n      ...SimilarProfessors_teacher\n      ...TeacherRatingTabs_teacher\n    }\n    id\n  }\n}\n\nfragment StickyHeaderContent_teacher on Teacher {\n  ...HeaderDescription_teacher\n  ...HeaderRateButton_teacher\n}\n\nfragment RatingDistributionWrapper_teacher on Teacher {\n  ...NoRatingsArea_teacher\n  ratingsDistribution {\n    total\n    ...RatingDistributionChart_ratingsDistribution\n  }\n}\n\nfragment TeacherInfo_teacher on Teacher {\n  id\n  lastName\n  numRatings\n  ...RatingValue_teacher\n  ...NameTitle_teacher\n  ...TeacherTags_teacher\n  ...NameLink_teacher\n  ...TeacherFeedback_teacher\n  ...RateTeacherLink_teacher\n  ...CompareProfessorLink_teacher\n}\n\nfragment SimilarProfessors_teacher on Teacher {\n  department\n  relatedTeachers {\n    legacyId\n    ...SimilarProfessorListItem_teacher\n    id\n  }\n}\n\nfragment TeacherRatingTabs_teacher on Teacher {\n  numRatings\n  courseCodes {\n    courseName\n    courseCount\n  }\n  ...RatingsList_teacher\n  ...RatingsFilter_teacher\n}\n\nfragment RatingsList_teacher on Teacher {\n  id\n  legacyId\n  lastName\n  numRatings\n  school {\n    id\n    legacyId\n    name\n    city\n    state\n    avgRating\n    numRatings\n  }\n  ...Rating_teacher\n  ...NoRatingsArea_teacher\n  ratings(first: 20) {\n    edges {\n      cursor\n      node {\n        ...Rating_rating\n        id\n        __typename\n      }\n    }\n    pageInfo {\n      hasNextPage\n      endCursor\n    }\n  }\n}\n\nfragment RatingsFilter_teacher on Teacher {\n  courseCodes {\n    courseCount\n    courseName\n  }\n}\n\nfragment Rating_teacher on Teacher {\n  ...RatingFooter_teacher\n  ...RatingSuperHeader_teacher\n  ...ProfessorNoteSection_teacher\n}\n\nfragment NoRatingsArea_teacher on Teacher {\n  lastName\n  ...RateTeacherLink_teacher\n}\n\nfragment Rating_rating on Rating {\n  comment\n  flagStatus\n  createdByUser\n  teacherNote {\n    id\n  }\n  ...RatingHeader_rating\n  ...RatingSuperHeader_rating\n  ...RatingValues_rating\n  ...CourseMeta_rating\n  ...RatingTags_rating\n  ...RatingFooter_rating\n  ...ProfessorNoteSection_rating\n}\n\nfragment RatingHeader_rating on Rating {\n  legacyId\n  date\n  class\n  helpfulRating\n  clarityRating\n  isForOnlineClass\n}\n\nfragment RatingSuperHeader_rating on Rating {\n  legacyId\n}\n\nfragment RatingValues_rating on Rating {\n  helpfulRating\n  clarityRating\n  difficultyRating\n}\n\nfragment CourseMeta_rating on Rating {\n  attendanceMandatory\n  wouldTakeAgain\n  grade\n  textbookUse\n  isForOnlineClass\n  isForCredit\n}\n\nfragment RatingTags_rating on Rating {\n  ratingTags\n}\n\nfragment RatingFooter_rating on Rating {\n  id\n  comment\n  adminReviewedAt\n  flagStatus\n  legacyId\n  thumbsUpTotal\n  thumbsDownTotal\n  thumbs {\n    thumbsUp\n    thumbsDown\n    computerId\n    id\n  }\n  teacherNote {\n    id\n  }\n  ...Thumbs_rating\n}\n\nfragment ProfessorNoteSection_rating on Rating {\n  teacherNote {\n    ...ProfessorNote_note\n    id\n  }\n  ...ProfessorNoteEditor_rating\n}\n\nfragment ProfessorNote_note on TeacherNotes {\n  comment\n  ...ProfessorNoteHeader_note\n  ...ProfessorNoteFooter_note\n}\n\nfragment ProfessorNoteEditor_rating on Rating {\n  id\n  legacyId\n  class\n  teacherNote {\n    id\n    teacherId\n    comment\n  }\n}\n\nfragment ProfessorNoteHeader_note on TeacherNotes {\n  createdAt\n  updatedAt\n}\n\nfragment ProfessorNoteFooter_note on TeacherNotes {\n  legacyId\n  flagStatus\n}\n\nfragment Thumbs_rating on Rating {\n  id\n  comment\n  adminReviewedAt\n  flagStatus\n  legacyId\n  thumbsUpTotal\n  thumbsDownTotal\n  thumbs {\n    computerId\n    thumbsUp\n    thumbsDown\n    id\n  }\n  teacherNote {\n    id\n  }\n}\n\nfragment RateTeacherLink_teacher on Teacher {\n  legacyId\n  numRatings\n  lockStatus\n}\n\nfragment RatingFooter_teacher on Teacher {\n  id\n  legacyId\n  lockStatus\n  isProfCurrentUser\n  ...Thumbs_teacher\n}\n\nfragment RatingSuperHeader_teacher on Teacher {\n  firstName\n  lastName\n  legacyId\n  school {\n    name\n    id\n  }\n}\n\nfragment ProfessorNoteSection_teacher on Teacher {\n  ...ProfessorNote_teacher\n  ...ProfessorNoteEditor_teacher\n}\n\nfragment ProfessorNote_teacher on Teacher {\n  ...ProfessorNoteHeader_teacher\n  ...ProfessorNoteFooter_teacher\n}\n\nfragment ProfessorNoteEditor_teacher on Teacher {\n  id\n}\n\nfragment ProfessorNoteHeader_teacher on Teacher {\n  lastName\n}\n\nfragment ProfessorNoteFooter_teacher on Teacher {\n  legacyId\n  isProfCurrentUser\n}\n\nfragment Thumbs_teacher on Teacher {\n  id\n  legacyId\n  lockStatus\n  isProfCurrentUser\n}\n\nfragment SimilarProfessorListItem_teacher on RelatedTeacher {\n  legacyId\n  firstName\n  lastName\n  avgRating\n}\n\nfragment RatingValue_teacher on Teacher {\n  avgRating\n  numRatings\n  ...NumRatingsLink_teacher\n}\n\nfragment NameTitle_teacher on Teacher {\n  id\n  firstName\n  lastName\n  department\n  school {\n    legacyId\n    name\n    id\n  }\n  ...TeacherDepartment_teacher\n  ...TeacherBookmark_teacher\n}\n\nfragment TeacherTags_teacher on Teacher {\n  lastName\n  teacherRatingTags {\n    legacyId\n    tagCount\n    tagName\n    id\n  }\n}\n\nfragment NameLink_teacher on Teacher {\n  isProfCurrentUser\n  id\n  legacyId\n  firstName\n  lastName\n  school {\n    name\n    id\n  }\n}\n\nfragment TeacherFeedback_teacher on Teacher {\n  numRatings\n  avgDifficulty\n  wouldTakeAgainPercent\n}\n\nfragment CompareProfessorLink_teacher on Teacher {\n  legacyId\n}\n\nfragment TeacherDepartment_teacher on Teacher {\n  department\n  departmentId\n  school {\n    legacyId\n    name\n    id\n  }\n}\n\nfragment TeacherBookmark_teacher on Teacher {\n  id\n  isSaved\n}\n\nfragment NumRatingsLink_teacher on Teacher {\n  numRatings\n  ...RateTeacherLink_teacher\n}\n\nfragment RatingDistributionChart_ratingsDistribution on ratingsDistribution {\n  r1\n  r2\n  r3\n  r4\n  r5\n}\n\nfragment HeaderDescription_teacher on Teacher {\n  id\n  legacyId\n  firstName\n  lastName\n  department\n  school {\n    legacyId\n    name\n    city\n    state\n    id\n  }\n  ...TeacherTitles_teacher\n  ...TeacherBookmark_teacher\n  ...RateTeacherLink_teacher\n  ...CompareProfessorLink_teacher\n}\n\nfragment HeaderRateButton_teacher on Teacher {\n  ...RateTeacherLink_teacher\n  ...CompareProfessorLink_teacher\n}\n\nfragment TeacherTitles_teacher on Teacher {\n  department\n  school {\n    legacyId\n    name\n    id\n  }\n}\n"#;
