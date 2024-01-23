pub mod authorization;
pub mod folders;
pub mod lists;
pub mod spaces;
pub mod tasks;
pub mod teams;
pub mod comments;
pub mod custom_task_types;
pub mod custom_fields;

use authorization::AuthorizationTraitTransporter;
use folders::FoldersTraitTransporter;
use lists::ListsTraitTransporter;
use spaces::SpacesTraitTransporter;
use tasks::TasksTraitTransporter;
use teams::TeamsTraitTransporter;
use comments::CommentsTraitTransporter;
use custom_task_types::CustomTaskTypesTraitTransporter;
use custom_fields::CustomFieldsTraitTransporter;

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
}

impl Api {
    pub fn new(transport: &crate::transport::Transport) -> Api {
        Api {
            teams: TeamsTraitTransporter::new(transport),
            spaces: SpacesTraitTransporter::new(transport.clone()),
            folders: FoldersTraitTransporter::new(transport.clone()),
            lists: ListsTraitTransporter::new(transport.clone()),
            tasks: TasksTraitTransporter::new(transport.clone()),
            authorization: AuthorizationTraitTransporter::new(transport.clone()),
            comments: CommentsTraitTransporter::new(transport.clone()),
            custom_task_types: CustomTaskTypesTraitTransporter::new(transport.clone()),
            custom_fields: CustomFieldsTraitTransporter::new(transport.clone()),
        }
    }

    pub fn teams(&self) -> &teams::TeamsTraitTransporter {
        &self.teams
    }

    pub fn spaces(&self) -> &spaces::SpacesTraitTransporter {
        &self.spaces
    }
}
