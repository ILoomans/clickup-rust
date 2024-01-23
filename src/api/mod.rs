pub mod authorization;
pub mod folders;
pub mod lists;
pub mod spaces;
pub mod tasks;
pub mod teams;
pub mod comments;

use authorization::AuthorizationTraitTransporter;
use folders::FoldersTraitTransporter;
use lists::ListsTraitTransporter;
use spaces::SpacesTraitTransporter;
use tasks::TasksTraitTransporter;
use teams::TeamsTraitTransporter;
use comments::CommentsTraitTransporter;

pub struct Api {
    pub teams: TeamsTraitTransporter,
    pub spaces: SpacesTraitTransporter,
    pub folders: FoldersTraitTransporter,
    pub lists: ListsTraitTransporter,
    pub tasks: TasksTraitTransporter,
    pub authorization: AuthorizationTraitTransporter,
    pub comments: CommentsTraitTransporter,
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
        }
    }

    pub fn teams(&self) -> &teams::TeamsTraitTransporter {
        &self.teams
    }

    pub fn spaces(&self) -> &spaces::SpacesTraitTransporter {
        &self.spaces
    }
}
