
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WidgetMeta {
  id: String,
  x: f32,
  y: f32,
  title: String
}


