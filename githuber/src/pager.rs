pub struct Pager {
	pub per_page: u32,
	pub page: u32,
}
impl Pager {
	pub fn query(&self) -> String {
		format!("per_page={}&page={}", self.per_page, self.page)
	}
}
