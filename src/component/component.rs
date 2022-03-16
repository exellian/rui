use crate::component::base::BaseComponent;
use crate::component::border::BorderComponent;
use crate::component::composition::CompositionComponent;
use crate::component::image::ImageComponent;
use crate::component::text::TextComponent;

pub enum Component {
    Rectangle(BaseComponent),
    Border(BaseComponent, BorderComponent),
    Composition(BaseComponent, CompositionComponent),
    Image(BaseComponent, ImageComponent),
    Text(BaseComponent, TextComponent)
}