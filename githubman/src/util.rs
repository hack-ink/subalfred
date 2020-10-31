#[macro_export]
macro_rules! api_queries {
	($self:ident, [$($path_part:ident),+], [$($query:ident),+]) => {{
		format!(
			"{}?{}",
			$crate::api!($self, [$($path_part),+]),
			$crate::queries!($self, [$($query),+])
			)
		}};
}

#[macro_export]
macro_rules! api {
	($self:ident, $([$($path_part:ident),+])?) => {{
		format!(
			"{}{}",
			$crate::GithubMan::API_BASE_URL,
			Self::PATH
				$($(
					.replace(&format!("{{{}}}", stringify!($path_part)), &$self.$path_part)
				)+)?
		)
	}}
}

#[macro_export]
macro_rules! queries {
	($self:ident, [$($query:ident),+]) => {{
		let mut queries = String::new();

		$(
			if let Some($query) = $self.$query {
				queries.push_str(&format!("{}={}&", stringify!($query), $query));
				}
		)+

		queries.trim_end_matches('&').to_owned()
	}};
}
