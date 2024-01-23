use serde::{Deserialize, Serialize};

mod folders;
mod lists;
mod spaces;
mod tasks;
mod teams;

mod authorization;

mod comments;

mod custom_task_types;
mod custom_fields;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyResponse {}

pub use self::authorization::{AccessToken, AuthorizedUser};
pub use self::comments::{Comment, Comments, CreateComment,CreateCommentResponse, CreateChatViewComment, UpdateComment};
pub use self::folders::{CreateFolder, Folder, Folders};
pub use self::lists::{CreateList, List, Lists};
pub use self::spaces::{CreateFeatureSpace, CreateSpace, EnabledStruct, Space, Spaces};
pub use self::tasks::{CreateTask, Task, Tasks};
pub use self::teams::{CreateTeam, Team, Teams};
pub use self::custom_task_types::CustomItems;
pub use self::custom_fields::Fields;
