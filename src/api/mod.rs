pub mod folders;
pub mod spaces;
pub mod teams;
pub mod lists;


use folders::FoldersTraitTransporter;
use spaces::SpacesTraitTransporter;
use teams::TeamsTraitTransporter;
use lists::ListsTraitTransporter;

pub struct Api {
    pub teams: TeamsTraitTransporter,
    pub spaces: SpacesTraitTransporter,
    pub folders: FoldersTraitTransporter,
    pub lists: ListsTraitTransporter,
}

impl Api {
    pub fn new(transport: &crate::transport::Transport) -> Api {
        Api {
            teams: TeamsTraitTransporter::new(transport),
            spaces: SpacesTraitTransporter::new(transport.clone()),
            folders: FoldersTraitTransporter::new(transport.clone()),
            lists: ListsTraitTransporter::new(transport.clone()),
        }
    }

    pub fn teams(&self) -> &teams::TeamsTraitTransporter {
        &self.teams
    }

    pub fn spaces(&self) -> &spaces::SpacesTraitTransporter {
        &self.spaces
    }


}

// pub struct Transporter {
//     transport: crate::transport::Transport,
// }

// impl Transporter {
//     pub fn new(transport: &crate::transport::Transport) -> Transporter {
//         Transporter {
//             transport: transport.clone(),
//         }
//     }
// }
// pub mod spaces;
// pub mod teams;
// pub mod folders;
// pub struct Api {
//     pub teams: Transporter,

//     pub spaces: Transporter,

//     pub folders: Transporter,
// }

// impl Api {
//     pub fn new(transport: &crate::transport::Transport) -> Api {
//         Api {
//             teams: teams::TeamsTrait::new(transport.clone()),
//             spaces: spaces::SpacesTrait::new(transport.clone()),
//             folders: folders::FoldersTrait::new(transport.clone()),

//         }
//     }

//     pub fn teams(&self) -> &Transporter {
//         &self.teams
//     }

//     pub fn spaces(&self) -> &Transporter {
//         &self.spaces
//     }
// }

// pub struct Transporter {
//     transport: crate::transport::Transport,
// }
