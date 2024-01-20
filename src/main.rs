pub mod api;
pub mod transport;
pub mod types;

use derive_more::{Display, From};

use serde_json::Error as SerdeError;

use crate::types::CreateSpace;

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
    // println!("Hello, world!");
    let key = dotenv::var("CLICKUP_API_KEY").unwrap();
    let click_up_api = api::Api::new(&transport::Transport::new(key.to_string()));

    // get teams
    let teams = click_up_api.teams.get_teams().unwrap();
    println!("Teams: {}", teams.teams.len());
    let memberid = teams.teams[0].members[1].user.id;
    let teamid = &teams.teams[0].id;

    let space = CreateSpace {
        name: "New Space Test".to_string(),
        multiple_assignees: true,
    };

    let spaces = click_up_api.spaces.get_spaces(teamid).unwrap();

    // println!("Team ID: {}", teamid);
    // click_up_api
    //     .teams
    //     .create_team(teamid, "New Team Test", [memberid.to_string()].to_vec())
    //     .unwrap();
    // println!("Teams: {}", teams.teams.len());

    // let task_id = "32t2d0z";
    // let task = click_up_api.tasks.get_task(task_id).unwrap();
    // println!("Task: {:?}", task);

    // let list = click_up_api.lists.get_list("176667743").unwrap();
    // print!("List: {:?}", list);

    // let folder_id = "31902507";
    // let folder = click_up_api.folders.get_folder(folder_id).unwrap();
    // println!("Folder: {:?}", folder);

    // let team_id = "31902507";

    // let teams = click_up_api.teams.get_teams().unwrap();
    // println!("Teams: {:?}", teams);
    // // let tasks = click_up_api.tasks.get_tasks(list_id).unwrap();
    // println!("Tasks: {:?}", tasks.tasks.len());
    // getAllLists();
}

fn getAllLists() {
    let key = dotenv::var("CLICKUP_API_KEY").unwrap();
    let click_up_api = api::Api::new(&transport::Transport::new(key.to_string()));

    let output = click_up_api.teams.get_teams().unwrap();
    let teamid = &output.teams[0].id;
    println!("Team ID: {}", teamid);

    let spaces = click_up_api.spaces.get_spaces(teamid).unwrap();
    // println!("Spaces: {:?}", spaces.spaces.len());

    for space in spaces.spaces {
        // println!("Space: {}", space.id);
        let folders = click_up_api.folders.get_folders(&space.id).unwrap();
        // println!("Folders: {:?}", folders.folders.len());
        for folder in folders.folders {
            // println!("Folder: {}", folder.id);
            // Don't Fail on an error
            let lists = click_up_api.lists.get_lists(&folder.id).unwrap();
            // println!("Lists: {:?}", lists.lists.len());
            for list in lists.lists {
                // println!("List: {}", list.id);
                // Lets get the tasks
                let tasks = click_up_api.tasks.get_tasks(&list.id).unwrap();
                // println!("Tasks: {:?}", tasks.tasks.len());
            }
        }
    }

    println!("Goodbye, world!");
}
