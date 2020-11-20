// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
use serde::Serialize;
// --- githuber ---
use crate::{api, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct CreateAnIssue {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// title	string	body
	/// Required. The title of the issue.
	#[builder(setter(into))]
	pub title: String,
	/// body	string	body
	/// The contents of the issue.
	#[builder(default)]
	pub body: Option<String>,
	// TODO: deprecated
	// /// assignee	string or null	body
	// /// Login for the user that this issue should be assigned to. NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. This field is deprecated.
	// #[builder(default)]
	// pub assignee: Option<String>,
	/// milestone	integer or null	body
	/// The number of the milestone to associate this issue with. NOTE: Only users with push access can set the milestone for new issues. The milestone is silently dropped otherwise.
	#[builder(default)]
	pub milestone: Option<u32>,
	/// labels	array of undefineds	body
	/// Labels to associate with this issue. NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise.
	#[builder(default)]
	pub labels: Option<Vec<String>>,
	/// assignees	array of strings	body
	/// Logins for Users to assign to this issue. NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise.
	#[builder(default)]
	pub assignees: Option<Vec<String>>,
}
impl GithubApi<Vec<u8>> for CreateAnIssue {
	const HTTP_METHOD: HttpMethod = HttpMethod::POST;
	const PATH: &'static str = "/repos/{owner}/{repo}/issues";
	const ACCEPT: &'static str = "application/vnd.github.v3+json";

	fn build_uri(&self) -> Uri {
		api!(self, [owner, repo]).parse().unwrap()
	}

	fn build_body(&self) -> Vec<u8> {
		serde_json::to_vec(&Body {
			title: &self.title,
			body: &self.body,
			milestone: &self.milestone,
			labels: &self.labels,
			assignees: &self.assignees,
		})
		.unwrap()
	}
}

#[derive(Serialize)]
struct Body<'a> {
	title: &'a String,
	#[serde(skip_serializing_if = "Option::is_none")]
	body: &'a Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	milestone: &'a Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	labels: &'a Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	assignees: &'a Option<Vec<String>>,
}
