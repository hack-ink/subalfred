#[macro_export]
macro_rules! uri {
	($self:ident, $([$($path_part:ident),+])?$(, [$($query:ident),+])?) => {{
		let api = $crate::api!($self, $([$($path_part),+])?);
		let queries = $crate::queries!($self, $([$($query),+])?);

		if queries.is_empty() {
			api
		} else {
			format!("{}?{}", api, queries)
		}
	}}
}

#[macro_export]
macro_rules! api {
	($self:ident, $([$($path_part:ident),+])?) => {{
		format!(
			"{}{}",
			$crate::Githubman::API_BASE_URL,
			Self::PATH
				$($(
					.replace(&format!("{{{}}}", stringify!($path_part)), &$self.$path_part)
				)+)?
		)
	}}
}

#[macro_export]
macro_rules! queries {
	($self:ident, $([$($query:ident),+])?) => {{
		#[allow(unused_mut)]
		let mut queries = ::std::string::String::new();

		$($(
			if let Some($query) = &$self.$query {
				queries.push_str(&format!("{}={}&", stringify!($query), $query));
				}
		)+)?

		queries.trim_end_matches('&').to_owned()
	}};
}
