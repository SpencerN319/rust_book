pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_by_ref(a: &i32, b: &i32) -> i32 {
    a + b
}

pub fn concatenate(s1: &mut String, s2: &String) {
    s1.push_str(s2)
}

pub fn concatenate_owned(mut s1: String, s2: &String) -> String {
    String::from(s1.push_str(s2))
}


