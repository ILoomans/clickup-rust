pub mod api;
pub mod transport;
pub mod types;
use dotenv::dotenv;

use derive_more::{Display, From};


use serde_json::Error as SerdeError;

// https://clickup.com/api/developer-portal/errors/
#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    /// Not Authorized
    #[display(fmt = "Team is not authorized")]
    NotAuthorized,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::Error::*;
        match *self {
            NotAuthorized => None,
        }
    }
}

//
impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::NotAuthorized
    }
}



fn main() {

    let key = dotenv::var("CLICKUP_API_KEY").unwrap();
    let click_up_api = api::Api::new(&transport::Transport::new(
        key.to_string(),
    ));

    let output = click_up_api.teams.get_teams().unwrap();
    let teamid = &output.teams[0].id;
    println!("Team ID: {}", teamid);

    let spaces = click_up_api.spaces.get_spaces(teamid).unwrap();
    println!("Spaces: {:?}", spaces.spaces.len());

    for space in spaces.spaces {
        println!("Space: {}", space.id);
        let folders = click_up_api.folders.get_folders(&space.id).unwrap();
        println!("Folders: {:?}", folders.folders.len());
        for folder in folders.folders {
            println!("Folder: {}", folder.id);
            // Don't Fail on an error
            let lists = click_up_api.lists.get_lists(&folder.id).unwrap();
            println!("Lists: {:?}", lists.lists.len());
            for list in lists.lists {
                println!("List: {}", list.id);
                // Lets get the tasks
            }
        }
    }

    println!("Goodbye, world!");
}
