use web_sys::Url;
use wasm_bindgen::prelude::*;
use shared::domain::jig::ModuleKind;
use crate::firebase::FirebaseUserInfo;
use serde::{Serialize, Deserialize};

pub type Id = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    NotFound,
    NoAuth,
    User(UserRoute),
    Admin(AdminRoute),
    Jig(JigRoute),
    Module(ModuleRoute),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRoute {
    Profile(ProfileSection),
    ContinueRegistration(FirebaseUserInfo),
    Signin,
    Register,
    SendEmailConfirmation,
    GotEmailConfirmation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileSection {
    Landing,
    ChangeEmail
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminRoute {
    Categories,
    Images,
    ImageAdd,
    ImageEdit(Id),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigRoute {
    Gallery,
    Edit(Id, Option<Id>),
    Play(Id, Option<Id>) 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigPlayMode {
    Producer,
    Audience 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleRoute {
    Edit(ModuleKind, Id, Id),
    Play(ModuleKind, Id, Id),
}

//Just for serializing across local routes
#[derive(Serialize, Deserialize)]
struct FirebaseUserQuery {
    pub user: String  //json-encoded FirebaseUserInfo
}

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_throw().location();
        let s:String = self.into();
        location.set_href(&s).unwrap_throw();
    }

    pub fn from_url(url:&str) -> Self {
        let url = Url::new(&url).unwrap_throw();
        let paths = url.pathname();
        let paths = paths.split("/").into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();
        let params = url.search_params();

        match paths {
            ["user", "profile"] => Self::User(UserRoute::Profile(ProfileSection::Landing)),
            ["user", "profile", "change-email"] => Self::User(UserRoute::Profile(ProfileSection::ChangeEmail)),
            ["user", "signin"] => Self::User(UserRoute::Signin),
            ["user", "register"] => Self::User(UserRoute::Register),
            ["user", "continue-registration"] => {
                if let Some(user) = params.get("user") {
                    let user:FirebaseUserInfo = serde_json::from_str(&user).unwrap_throw();
                    Self::User(UserRoute::ContinueRegistration(user))
                } else {
                    Self::NoAuth
                }
            }
            ["user", "send-email-confirmation"] => Self::User(UserRoute::SendEmailConfirmation),
            ["user", "got-email-confirmation"] => Self::User(UserRoute::GotEmailConfirmation),
            ["admin", "categories"] => Self::Admin(AdminRoute::Categories),
            ["admin", "images"] => Self::Admin(AdminRoute::Images),
            ["admin", "image-add"] => Self::Admin(AdminRoute::ImageAdd),
            ["admin", "image-edit", id] => Self::Admin(AdminRoute::ImageEdit(id.to_string())),
            ["jig", "gallery"] => Self::Jig(JigRoute::Gallery),
            ["jig", "edit", jig_id] => Self::Jig(JigRoute::Edit(jig_id.to_string(), None)),
            ["jig", "edit", jig_id, module_id] => Self::Jig(JigRoute::Edit(jig_id.to_string(), Some(module_id.to_string()))),
            ["jig", "play", jig_id] => Self::Jig(JigRoute::Play(jig_id.to_string(), None)),
            ["jig", "play", jig_id, module_id] => Self::Jig(JigRoute::Play(jig_id.to_string(), Some(module_id.to_string()))),
            ["module", kind, "edit", jig_id, module_id] => Self::Module(ModuleRoute::Edit(module_kind_from_str(kind).expect_throw("unknown module kind!"), jig_id.to_string(), module_id.to_string())),
            ["module", kind, "play", jig_id, module_id] => Self::Module(ModuleRoute::Play(module_kind_from_str(kind).expect_throw("unknown module kind!"), jig_id.to_string(), module_id.to_string())),
            ["no-auth"] => Self::NoAuth,

            _ => Self::NotFound
        }
    }
}

pub fn module_kind_from_str(kind:&str) -> Option<ModuleKind> {
    match kind {
        "poster" => Some(ModuleKind::Poster),
        "design-page" => Some(ModuleKind::DesignPage),
        "memory-game" => Some(ModuleKind::MemoryGame),
        _ => None
    }
}

pub fn module_kind_to_str(kind:ModuleKind) -> &'static str {
    match kind {
        ModuleKind::Poster => "poster",
        ModuleKind::DesignPage => "design-page",
        ModuleKind::MemoryGame => "memory-game",
    }
}

pub fn module_kind_to_label(kind:ModuleKind) -> &'static str {
    match kind {
        ModuleKind::Poster => "Poster",
        ModuleKind::DesignPage => "Design",
        ModuleKind::MemoryGame => "Memory Game",
    }
}
impl From<Route> for String {
    fn from(route:Route) -> Self {
        match route {
            Route::NoAuth => "/no-auth".to_string(),

            Route::User(route) => {
                match route {
                    UserRoute::Profile(ProfileSection::Landing) => "/user/profile".to_string(),
                    UserRoute::Profile(ProfileSection::ChangeEmail) => "/user/profile/change-email".to_string(),
                    UserRoute::ContinueRegistration(user) => {
                        let user = serde_json::to_string(&user).unwrap_throw();
                        let query = FirebaseUserQuery { user };
                        let query = serde_qs::to_string(&query).unwrap_throw();
                        format!("/user/continue-registration?{}", query) 
                    }
                    UserRoute::Signin => "/user/signin".to_string(),
                    UserRoute::Register => "/user/register".to_string(),
                    UserRoute::SendEmailConfirmation => "/user/send-email-confirmation".to_string(),
                    UserRoute::GotEmailConfirmation => "/user/got-email-confirmation".to_string(),
                }
            },
            Route::Admin(route) => {
                match route {
                    AdminRoute::Categories => "/admin/categories".to_string(),
                    AdminRoute::Images => "/admin/images".to_string(),
                    AdminRoute::ImageAdd => "/admin/image-add".to_string(),
                    AdminRoute::ImageEdit(id) => format!("/admin/image-edit/{}", id),
                }
            },
            Route::Jig(route) => {
                match route {
                    JigRoute::Gallery => "/jig/gallery".to_string(),
                    JigRoute::Edit(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/jig/edit/{}/{}", jig_id, module_id)
                        } else {
                            format!("/jig/edit/{}", jig_id)
                        }
                    }
                    JigRoute::Play(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/jig/play/{}/{}", jig_id, module_id)
                        } else {
                            format!("/jig/play/{}", jig_id)
                        }
                    }
                }
            },
            Route::Module(route) => {
                match route {
                    ModuleRoute::Edit(kind, jig_id, module_id) => format!("/module/{}/edit/{}/{}", module_kind_to_str(kind), jig_id, module_id),
                    ModuleRoute::Play(kind, jig_id, module_id) => format!("/module/{}/play/{}/{}", module_kind_to_str(kind), jig_id, module_id),
                }
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}

