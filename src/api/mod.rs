/// The Authorization API.
pub mod authorization;
/// The Comments API.
pub mod comments;
/// The Custom Fields API.
pub mod custom_fields;
/// The Custom Task Types API.
pub mod custom_task_types;
/// The Folders API.
pub mod folders;
/// The Goals API.
pub mod goals;
/// The Lists API.
pub mod lists;
/// The Members API.
pub mod members;
/// The Spaces API.
pub mod spaces;
/// The Tasks API.   
pub mod tasks;
/// The Teams API.
pub mod teams;

use authorization::AuthorizationTraitTransporter;
use comments::CommentsTraitTransporter;
use custom_fields::CustomFieldsTraitTransporter;
use custom_task_types::CustomTaskTypesTraitTransporter;
use folders::FoldersTraitTransporter;
use goals::GoalsTraitTransporter;
use lists::ListsTraitTransporter;
use members::MembersTraitTransporter;
use spaces::SpacesTraitTransporter;
use tasks::TasksTraitTransporter;
use teams::TeamsTraitTransporter;

pub struct Api {
    pub teams: TeamsTraitTransporter,
    pub spaces: SpacesTraitTransporter,
    pub folders: FoldersTraitTransporter,
    pub lists: ListsTraitTransporter,
    pub tasks: TasksTraitTransporter,
    pub authorization: AuthorizationTraitTransporter,
    pub comments: CommentsTraitTransporter,
    pub custom_task_types: CustomTaskTypesTraitTransporter,
    pub custom_fields: CustomFieldsTraitTransporter,
    pub goals: GoalsTraitTransporter,
    pub members: MembersTraitTransporter,
}

/// The ClickUp API and its methods.
impl Api {
    /// Create a new instance of the ClickUp API.
    pub fn new(api_key: String) -> Api {
        let transport = crate::transport::Transport::new(api_key);
        Api {
            teams: TeamsTraitTransporter::new(&transport),
            spaces: SpacesTraitTransporter::new(transport.clone()),
            folders: FoldersTraitTransporter::new(transport.clone()),
            lists: ListsTraitTransporter::new(transport.clone()),
            tasks: TasksTraitTransporter::new(transport.clone()),
            authorization: AuthorizationTraitTransporter::new(transport.clone()),
            comments: CommentsTraitTransporter::new(transport.clone()),
            custom_task_types: CustomTaskTypesTraitTransporter::new(transport.clone()),
            custom_fields: CustomFieldsTraitTransporter::new(transport.clone()),
            goals: GoalsTraitTransporter::new(transport.clone()),
            members: MembersTraitTransporter::new(transport.clone()),
        }
    }
}
