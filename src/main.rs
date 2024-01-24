pub mod api;
pub mod transport;
pub mod types;

use api::custom_fields;
use derive_more::{Display, From};

use serde_json::Error as SerdeError;
use types::{CreateGoal, UpdateGoal};

use crate::types::{CreateChatViewComment, CreateComment, CreateList, CreateSpace, UpdateComment};

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

// TODO: check that the status is available to the list
// TODO: Update for team
fn main() {
    // println!("Hello, world!");
    let key = dotenv::var("CLICKUP_API_KEY").unwrap();
    let click_up_api = api::Api::new(&transport::Transport::new(key.to_string()));

    let team_id = 2662797;
    let member_id = 6647942;



    let goal = CreateGoal {
        name: "New Goal".to_string(),
        due_date: 1737722728,        
        description: "This is a new goal".to_string(),
        multiple_owners: false,
        owners: [member_id].to_vec(),
        color: "#32a852".to_string(),

    };

    let goal = click_up_api.goals.create_goal(team_id, goal).unwrap();
    let id = goal.goal.id;
    let updated_goal = UpdateGoal {
        name: "New Goal Updated".to_string(),
        due_date: 1737722728,        
        description: "This is a new goal".to_string(),
        add_owners: [].to_vec(),
        rem_owners: [].to_vec(),
        color: "#32a852".to_string(),
    };

    click_up_api.goals.update_goal(&id, updated_goal).unwrap();

    let updatedGoal = click_up_api.goals.get_goal(&id).unwrap();
    println!("Updated Goal: {:?}", updatedGoal);

    
    // let goals = click_up_api.goals.get_goals(team_id).unwrap();
    // println!("Goals: {:?}", goals);

    // let goal_id = &goals.goals[0].id;

    // let goal = click_up_api.goals.get_goal(goal_id).unwrap();
    // println!("Goal: {:?}", goal);

    // click_up_api.goals.delete_goal(goal_id).unwrap();


    // let list_id = 176667743;
    // let custom_fields = click_up_api.custom_fields.get_accessible_custom_fields(list_id).unwrap();
    // println!("Custom Fields: {:?}", custom_fields);


    


    // let authorized_user = click_up_api.authorization.get_authorized_user().unwrap();
    // println!("Authorized User: {:?}", authorized_user);


    // let custom_task_types = click_up_api.custom_task_types.get_custom_task_types(team_id).unwrap();

    // println!("Custom Task Types: {:?}", custom_task_types);



    // let list_id: u64 = 901201442725;

    //    let comment = CreateComment {
    //     comment_text: "This is a test comment".to_string(),
    //     assignee: 6647942,
    //     notify_all: false
    // }; 

    // click_up_api.comments.create_list_comment(list_id, comment).unwrap();


    // let comments = click_up_api.comments.get_list_comments(list_id).unwrap();
    // println!("Comments: {:?}", comments);

    // let comment_id = comments.comments[0].id.parse::<u64>().unwrap();

    // let updaed_comment = UpdateComment {
    //     comment_text: "This is an updated comment".to_string(),
    //     resolved: false,
    //     notify_all: false
    // };

    // click_up_api.comments.update_comment(comment_id, updaed_comment).unwrap();

    // click_up_api.comments.delete_comment(comment_id).unwrap();


    // let new_comments = click_up_api.comments.get_list_comments(list_id).unwrap();
    // println!("New Comments: {:?}", new_comments);





    
    // let view_id = "2h8cd-1872";
    
    // let comment = CreateChatViewComment {
    //     comment_text: "This is a test comment".to_string(),
    //     notify_all: false
    // };

    // click_up_api.comments.create_chat_view_comment(view_id, comment).unwrap();







    // let member_id = "6647942";
    // let task_id = "8693kdpp2";
    // let team_id = 2662797;
    // let custom_task_ids = false;


    // let comment = CreateComment {
    //     comment_text: "This is a test comment".to_string(),
    //     assignee: 6647942,
    //     notify_all: false
    // };
    // // create a comment 
    // click_up_api.comments.create_task_comment(task_id, false, team_id, comment).unwrap();






    // let comment = click_up_api.comments.get_task_comments("8693m01tx").unwrap();
    // println!("Comment: {:?}", comment);


    // let chat_view_comment = click_up_api.comments.get_chat_view_comments("2h8cd-1852").unwrap();
    // println!("Chat View Comment: {:?}", chat_view_comment);




    // let space_id = "90120392694";

    // let space = CreateSpace {
    //     name: "New Space Test Updated".to_string(),
    //     multiple_assignees: true,
    //     features: None,
    // };

    // let updatedSpace = click_up_api.spaces.update_space(space_id, space).unwrap();
    // println!("Updated Space: {:?}", updatedSpace);

    // let updatedSpace =

    // let space_id = "90120333090";

    // let output = click_up_api.spaces.delete_space(space_id).unwrap();
    // println!("Output: {:?}", output);

    // let space = click_up_api.spaces.get_space(space_id).unwrap();
    // println!("Space: {:?}", space);

    // let folder_id = "90120840978";
    // let folder = click_up_api.folders.get_folder(folder_id).unwrap();
    // println!("Folder: {:?}", folder);
    // let list = CreateList {
    //     name: "new list 2".to_string(),
    //     content: None,
    //     due_date: None,
    //     due_date_time: None,
    //     priority: None,
    //     assignee: None,
    //     status: None,
    // };

    ///////// UPDATE FOLDER WORK
    // first create a folder

    // let folder = click_up_api.folders.create_folder(space_id, "new folder").unwrap();
    // println!("Folder: {:?}", folder);

    // let folder_id = folder.id;

    // let updatedFolder = click_up_api.folders.update_folder(&folder_id, "new folder updated").unwrap();

    // println!("Updated Folder: {:?}", updatedFolder);
    //////// UPDATED LIST WORK
    // let folder_id = "90120842824";
    // let list = CreateList {
    //     name: "new list".to_string(),
    //     content: None,
    //     due_date: None,
    //     due_date_time: None,
    //     priority: None,
    //     assignee: None,
    //     status: None,
    // };

    // let createdList: types::List = click_up_api.lists.create_list(folder_id, list).unwrap();

    // let updated_list = CreateList {
    //     name: "new list 2 updated".to_string(),
    //     content: None,
    //     due_date: None,
    //     due_date_time: None,
    //     priority: None,
    //     assignee: None,
    //     status: None,
    // };

    // let updatedList: types::List = click_up_api
    //     .lists
    //     .update_list(&createdList.id, updated_list)
    //     .unwrap();
    // println!("Updated List: {:?}", updatedList);

    ///// Updated task work
    //
    // let list_id = "901201445324";

    // let task = types::CreateTask {
    //     name: "new task 9".to_string(),
    //     description: None,
    //     markdown_description: None,
    //     assignees: None,
    //     tags: None,
    //     status: None,
    //     priority: None,
    //     due_date: None,
    //     due_date_time: None,
    //     time_estimate: None,
    //     start_date: None,
    //     start_date_time: None,
    //     notify_all: None,
    //     parent: None,
    //     links_to: None,
    //     check_required_custom_fields: None,
    //     custom_fields: None,
    // };

    // let createdTask = click_up_api.tasks.create_task(list_id, task).unwrap();

    // let task_id = createdTask.id;

    // let updated_task = types::CreateTask {
    //     name: "new task 8 updated".to_string(),
    //     description: None,
    //     markdown_description: None,
    //     assignees: None,
    //     tags: None,
    //     status: Some("Complete".to_string()),
    //     priority: None,
    //     due_date: None,
    //     due_date_time: None,
    //     time_estimate: None,
    //     start_date: None,
    //     start_date_time: None,
    //     notify_all: None,
    //     parent: None,
    //     links_to: None,
    //     check_required_custom_fields: None,
    //     custom_fields: None,
    // };

    // let updatedTask = click_up_api.tasks.update_task(&task_id, updated_task).unwrap();

    // println!("Updated Task: {:?}", updatedTask);

    // println!("Created List: {:?}", createdList);

    // let list_id = createdList.id;
    // click_up_api.lists.delete_list(&list_id).unwrap();

    // let list_id = "901201442725";

    // let task = types::CreateTask {
    //     name: "new task 7".to_string(),
    //     description: None,
    //     markdown_description: None,
    //     assignees: None,
    //     tags: None,
    //     status: None,
    //     priority: None,
    //     due_date: None,
    //     due_date_time: None,
    //     time_estimate: None,
    //     start_date: None,
    //     start_date_time: None,
    //     notify_all: None,
    //     parent: None,
    //     links_to: None,
    //     check_required_custom_fields: None,
    //     custom_fields: None,
    // };

    // let createdTask = click_up_api.tasks.create_task(list_id, task).unwrap();
    // println!("Created Task: {:?}", createdTask);

    // let created_list_id = createdTask.id;

    // click_up_api.tasks.delete_task(&created_list_id).unwrap();

    // let folder_id = folder.id;
    // // delete the folder

    // click_up_api.folders.delete_folder(&folder_id).unwrap();

    // get teams
    // let teams = click_up_api.teams.get_teams().unwrap();
    // println!("Teams: {:?}", teams);
    // // let memberid = teams.teams[0].members[1].user.id;
    // let teamid = &teams.teams[0].id;

    // let space = CreateSpace {
    //     name: "New Space Test".to_string(),
    //     multiple_assignees: true,
    //     features: None,
    // };

    // let result = click_up_api.spaces.create_space(teamid, space).unwrap();
    // println!("Space: {:?}", result);

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
