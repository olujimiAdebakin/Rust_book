pub mod name_helpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name: String = format!("{0} {1}", first, last);
        return full_name;
    }
}

pub mod database_helpers {
    pub fn db_connection_string() -> String {
        let conn_str: String =
            String::from("Server=localhost;Database=mydb;User Id=myuser;Password=mypassword;");
        return conn_str;
    }
}

pub mod api_helpers {
    pub fn get_age_plus_5(age: i8) -> i8 {
        return age + 6;
    }
}
