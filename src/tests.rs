#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::{Status, Cookie};

    #[test]
    fn should_get_home() {
        let client = Client::new(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn should_redirect_on_get_new_recipe() {
        let client = Client::new(rocket()).unwrap();
        let response = client.get("/recipes/new").dispatch();
        assert_eq!(response.status(), Status::Forbidden);
    }

    // Test bellow does not work because currently we can't create private cookie
    // https://github.com/SergioBenitez/Rocket/issues/488
    // https://github.com/SergioBenitez/Rocket/pull/487
    // #[test]
    // fn should_not_redirect_on_get_new_recipe() {
    //     let client = Client::new(rocket()).unwrap();
    //     let request = client.get("/recipes/new").cookie(
    //         Cookie::new("user_id", "1"),
    //     );
    //     let response = request.dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    // }

}
