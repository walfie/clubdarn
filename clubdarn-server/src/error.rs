use clubdarn;

error_chain!{
    links {
        ClubDarn(clubdarn::Error, clubdarn::error::ErrorKind);
    }
}
