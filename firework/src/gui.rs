use crate::{
    client::Client,
    {ConnectionError, ServerHandler, ServerProxy},
};
use async_trait::async_trait;
use firework_protocol::{data_types::Slot, server_bound::ClickContainer};

pub struct GUIInit {
    pub title: String,
    pub window_type: WindowType,
    pub items: Vec<Slot>,
}

#[async_trait]
pub trait GuiScreen<Handler, Proxy>
where
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
    Proxy: ServerProxy + Send + Sync + 'static,
{
    async fn init(&self, client: &Client<Handler, Proxy>) -> Result<GUIInit, ConnectionError>;
    async fn handle_click(
        &self,
        slot: ClickContainer,
        client: &Client<Handler, Proxy>,
    ) -> Result<(), ConnectionError>;
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum WindowType {
    Generic9x1,
    Generic9x2,
    Generic9x3,
    Generic9x4,
    Generic9x5,
    Generic9x6,
    Generic3x3,
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    Crafting,
    Enchantment,
    Furnace,
    Grindstone,
    Hopper,
    Lectern,
    Loom,
    Merchant,
    ShulkerBox,
    Smithing,
    Smoker,
    CartographyTable,
    Stonecutter,
}

impl WindowType {
    pub fn len(&self) -> usize {
        match self {
            WindowType::Generic9x1 => 9,
            _ => 0,
        }
    }
}
