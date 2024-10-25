use rand::Rng;
use rand::thread_rng;



pub struct Reptilia {
    FakerGun: rand::rngs::ThreadRng,
}


pub trait TReptiliaI
{
    fn reptilia_boolean(&self) -> bool;
    fn reptilia_int(&self) -> i32;
    fn reptilia_string_max_length(&self, l: usize) -> String;
    fn reptilia_float_between(&self, max_decimals: i32, min: f64, max: f64) -> f64;
    fn reptilia_int_between(&self, min: i32, max: i32) -> i32;
    fn reptilia_uuid(&self) -> String;
    fn reptilia_current_iso_timestamp(&self) -> String;
    fn reptilia_current_timestamp(&self) -> i64;
    fn reptilia_guid(&self) -> String;
    fn reptilia_address_longitude(&self) -> f64;
    fn reptilia_address_latitude(&self) -> f64;
    fn reptilia_address_country(&self) -> String;
    fn reptilia_address_street_address(&self) -> String;
    fn reptilia_address_street_name(&self) -> String;
    fn reptilia_address_city(&self) -> String;
    fn reptilia_person_name_suffix(&self) -> String;
    fn reptilia_person_name_prefix(&self) -> String;
    fn reptilia_person_full_name(&self) -> String;
    fn reptilia_person_last_name(&self) -> String;
    fn reptilia_person_first_name(&self) -> String;
    fn reptilia_user_agent(&self) -> String;
    fn reptilia_locale(&self) -> String;
    fn reptilia_password(&self) -> String;
    fn reptilia_mac_address(&self) -> String;
    fn reptilia_ip(&self) -> String;
    fn reptilia_safe_color_hex(&self) -> String;
    fn reptilia_safe_color_name(&self) -> String;
    fn reptilia_date_future(&self) -> String;
    fn reptilia_date_past(&self) -> String;
    fn reptilia_date_recent(&self) -> String;
    fn reptilia_lorem_word(&self) -> String;
    fn reptilia_lorem_words(&self) -> String;
    fn reptilia_lorem_sentence(&self) -> String;
    fn reptilia_phrase(&self) -> String;
    fn reptilia_lorem_sentences(&self) -> String;
    fn reptilia_lorem_lines(&self) -> String;
    fn reptilia_lorem_paragraph(&self) -> String;
    fn reptilia_lorem_text(&self) -> String;
    fn reptilia_lorem_paragraphs(&self) -> String;
    fn reptilia_lorem_slug(&self) -> String;
    fn reptilia_noun(&self) -> String;
    fn reptilia_verb(&self) -> String;
    fn reptilia_ing_verb(&self) -> String;
    fn reptilia_adjective(&self) -> String;
    fn reptilia_word(&self) -> String;
    fn reptilia_words(&self) -> String;
    fn reptilia_department(&self) -> String;
    fn reptilia_product_name(&self) -> String;
    fn reptilia_product_material(&self) -> String;
    fn reptilia_product_adjective(&self) -> String;
    fn reptilia_product(&self) -> String;
    fn reptilia_price(&self) -> String;
    fn reptilia_file_path(&self) -> String;
    fn reptilia_mime_type(&self) -> String;
    fn reptilia_directory_path(&self) -> String;
    fn reptilia_common_file_extension(&self) -> String;
    fn reptilia_common_file_type(&self) -> String;
    fn reptilia_common_file_name(&self) -> String;
    fn reptilia_file_type(&self) -> String;
    fn reptilia_file_extension(&self) -> String;
    fn reptilia_file_name(&self) -> String;
    fn reptilia_url(&self) -> String;
    fn reptilia_username(&self) -> String;
    fn reptilia_example_email(&self) -> String;
    fn reptilia_email(&self) -> String;
    fn reptilia_domain_word(&self) -> String;
    fn reptilia_domain_suffix(&self) -> String;
    fn reptilia_domain_name(&self) -> String;
    fn reptilia_month(&self) -> String;
    fn reptilia_weekday(&self) -> String;
    fn reptilia_database_column(&self) -> String;
    fn reptilia_database_type(&self) -> String;
    fn reptilia_database_collation(&self) -> String;
    fn reptilia_database_engine(&self) -> String;
    fn reptilia_catch_phrase(&self) -> String;
    fn reptilia_catch_phrase_adjective(&self) -> String;
}






