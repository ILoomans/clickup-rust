use serde::{Deserialize, Serialize};

mod authorization;
mod folders;
mod goals;
mod lists;
mod members;
mod spaces;
mod tasks;
mod teams;

mod comments;

mod custom_fields;
mod custom_task_types;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyResponse {}

pub use self::authorization::{AccessToken, AuthorizedUser};
pub use self::comments::{
    Comment, Comments, CreateChatViewComment, CreateComment, CreateCommentResponse, UpdateComment,
};
pub use self::custom_fields::Fields;
pub use self::custom_task_types::CustomItems;
pub use self::folders::{CreateFolder, Folder, Folders};
pub use self::goals::{CreateGoal, Goal, GoalContainer, Goals, UpdateGoal};
pub use self::lists::{CreateList, List, Lists};
pub use self::members::Members;
pub use self::spaces::{CreateFeatureSpace, CreateSpace, EnabledStruct, Space, Spaces};
pub use self::tasks::{CreateTask, Task, Tasks};
pub use self::teams::{CreateTeam, Team, Teams};
