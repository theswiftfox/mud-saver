pub struct MudrunnerSave<'a> {
    user_name: &'a str,
    file_hash: u64,
    original_name: &'a str,
}

// unction to get a vector of the mudrunner savegames' titles/user names in our app's storage
pub fn get_archived_mudrunner_saves<'a>() -> Vec<&'a str> {
    vec![""]
}


// function to get a vector of the mudrunner savegames' file names in Mudrunner's storage
pub fn get_available_mudrunner_saves<'a>() -> Vec<&'a str> {
    vec![""]
}

// function to archive a specific savegame to our app's storage
pub fn archive_savegame(savegame: &MudrunnerSave) -> Result<(), std::io::ErrorKind> {
    Err(std::io::ErrorKind::PermissionDenied)
}

// function to install a specific savegame (overwriting the existing one)
pub fn install_savegame(savegame: &MudrunnerSave) -> Result<(), std::io::ErrorKind> {
    Err(std::io::ErrorKind::PermissionDenied)
}