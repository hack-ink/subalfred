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
	}};
}

#[macro_export]
macro_rules! api {
	($self:ident, $([$($path_part:ident),+])?) => {{
		format!(
			"{}{}",
			$crate::Githuber::API_BASE_URL,
			Self::PATH
				$($(
					.replace($crate::api!($self, $path_part), &$self.$path_part)
				)+)?
		)
	}};
	($self:ident, r#ref) => {
		"{ref}"
	};
	($self:ident, $path_part:ident) => {
		&format!("{{{}}}", stringify!($path_part))
	};
}

#[macro_export]
macro_rules! queries {
	($self:ident, $([$($query:ident),+])?) => {{
		#[allow(unused_mut)]
		let mut queries = ::std::string::String::new();

		$($(
			$crate::queries!($self, $query, queries);
		)+)?

		queries.trim_end_matches('&').to_owned()
	}};
	($self:ident, r#ref, $queries:expr) => {
		if let Some(r#ref) = &$self.r#ref {
			$queries.push_str(&format!("ref={}&", r#ref));
			}
	};
	($self:ident, $query:ident, $queries:expr) => {
		if let Some($query) = &$self.$query {
			$queries.push_str(&format!("{}={}&", stringify!($query), $query));
			}
	};
}
