use serde::{Deserialize, Serialize};

mod folders;
mod lists;
mod spaces;
mod tasks;
mod teams;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmpptyResponse {}

pub use self::folders::{CreateFolder, Folder, Folders};
pub use self::lists::{CreateList, List, Lists};
pub use self::spaces::{CreateFeatureSpace, CreateSpace, EnabledStruct, Space, Spaces};
pub use self::tasks::{CreateTask, Task, Tasks};
pub use self::teams::{CreateTeam, Team, Teams};
