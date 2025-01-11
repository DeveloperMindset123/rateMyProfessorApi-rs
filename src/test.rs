// I need to be able to do something like the following
// this is more for reference
// the "class" should have two non-instantiated methopds
let college_structure = RateMyProfessor::construct_college("CUNY City College");
let college_structure_professor = RateMyProfessor::construct_college_and_professor("CUNY City College", "Douglas Troeger");

// then I should be able to invoke methods such as the following
// these should be the methods that should be instantiated on the other hand
college_structure.get_college_info();   // returns an array of json data
college_structure.set_new_college("Some Other College");        // inplace modification (void type)
college_structure.set_new_professor("Some Other Professor");    // inplace modification (void type)
college_structure.get_teacher_summary();    // returns an array of json data
college_structure.get_teacher_comments();   // returns an array of json data
college_structure.get_professor_list();     // TODO : implement this


// TODO : turn this into a feature as well
// '''
// "query TeacherSearchResultsPageQuery(\n  $query: TeacherSearchQuery!\n  $schoolID: ID\n  $includeSchoolFilter: Boolean!\n) {\n  search: newSearch {\n    ...TeacherSearchPagination_search_1ZLmLD\n  }\n  school: node(id: $schoolID) @include(if: $includeSchoolFilter) {\n    __typename\n    ... on School {\n      ...StickyHeaderContent_school\n    }\n    id\n  }\n}\n\nfragment TeacherSearchPagination_search_1ZLmLD on newSearch {\n  teachers(query: $query, first: 1000, after: \"\") {\n    didFallback\n    edges {\n      cursor\n      node {\n        ...TeacherCard_teacher\n        id\n        __typename\n      }\n    }\n    pageInfo {\n      hasNextPage\n      endCursor\n    }\n    resultCount\n    filters {\n      field\n      options {\n        value\n        id\n      }\n    }\n  }\n}\n\nfragment StickyHeaderContent_school on School {\n  name\n  ...HeaderDescription_school\n  ...HeaderRateButton_school\n}\n\nfragment HeaderDescription_school on School {\n  name\n  city\n  state\n  legacyId\n  ...RateSchoolLink_school\n  ...CompareSchoolLink_school\n}\n\nfragment HeaderRateButton_school on School {\n  ...RateSchoolLink_school\n  ...CompareSchoolLink_school\n}\n\nfragment RateSchoolLink_school on School {\n  legacyId\n}\n\nfragment CompareSchoolLink_school on School {\n  legacyId\n}\n\nfragment TeacherCard_teacher on Teacher {\n  id\n  legacyId\n  avgRating\n  numRatings\n  ...CardFeedback_teacher\n  ...CardSchool_teacher\n  ...CardName_teacher\n  ...TeacherBookmark_teacher\n}\n\nfragment CardFeedback_teacher on Teacher {\n  wouldTakeAgainPercent\n  avgDifficulty\n}\n\nfragment CardSchool_teacher on Teacher {\n  department\n  school {\n    name\n    id\n  }\n}\n\nfragment CardName_teacher on Teacher {\n  firstName\n  lastName\n}\n\nfragment TeacherBookmark_teacher on Teacher {\n  id\n  isSaved\n}\n"
// '''