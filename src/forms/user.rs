use super::super::schema::users;
use rocket::request::Form;

#[table_name = "users"]
#[derive(FromForm, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn from_form(form: Form<Self>) -> Self {
        Self {
            id: None,
            firstname: form.get().firstname.to_string(),
            lastname: form.get().lastname.to_string(),
            email: form.get().email.to_string(),
            password: Self::encrypt(form.get().password.to_string()),
        }
    }


    fn encrypt(password: String) -> String {
        use crypto::sha2::Sha256;
        use crypto::digest::Digest;

        let mut sha = Sha256::new();
        sha.input_str(&password);
        sha.result_str()
    }
}
