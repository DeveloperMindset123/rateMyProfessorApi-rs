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

pub struct RateMyProfessor {

}
