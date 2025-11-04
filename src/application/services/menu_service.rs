use crate::application::dtos::menu_dto::{CreateMenuRequest, MenuResponse};
use crate::domain::MenuRepository;
use crate::domain::entities::Menu;
use crate::shared::errors::error::Error;

pub struct MenuService<R: MenuRepository> {
    menu_repository: R,
}

impl<R: MenuRepository> MenuService<R> {
    pub fn new(repository: R) -> Self {
        Self {
            menu_repository: repository,
        }
    }

    pub async fn create_menu(&self, request: CreateMenuRequest) -> Result<MenuResponse, Error> {
        let mut menu = Menu::new(
            request.name,
            request.path,
            request.component,
            Option::from(request.icon),
            Option::from(request.parent_id),
            Option::from(request.order_num),
        )?;
        self.menu_repository.save(&mut menu).await?;
        Ok(MenuResponse::from(menu))
    }

    pub async fn get_menu_by_id(&self, id: i64) -> Result<MenuResponse, Error> {
        let menu = self.menu_repository.find_by_id(id).await?;
        match menu {
            None => Err(Error::NotFound("Menu not found".to_string())),
            Some(menu) => Ok(MenuResponse::from(menu)),
        }
    }

    pub async fn get_all_menus(&self) -> Result<Vec<MenuResponse>, Error> {
        let menus = self.menu_repository.get_all_menus().await?;
        Ok(menus
            .into_iter()
            .map(|menu| MenuResponse::from(menu))
            .collect())
    }
}
