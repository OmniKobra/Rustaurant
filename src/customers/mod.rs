use super::Order;

pub struct Customer<'a> {
    pub count: u32,
    pub order: Option<&'a Order<'a>>,
}
