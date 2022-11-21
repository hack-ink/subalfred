// hack-ink
use super::*;

#[tokio::test]
async fn track_update_should_work() {
	assert_eq!(
		track_update("hack-ink", "subalfred", "v0.9.0-rc17...v0.9.0-rc18").await.unwrap(),
		vec![
			PullRequest {
				title: "Release `subspector`".into(),
				html_url: "https://github.com/hack-ink/subalfred/pull/307".into(),
				labels: vec![Label { name: "CI/CD".into() }]
			},
			PullRequest {
				title: "Update dependencies".into(),
				html_url: "https://github.com/hack-ink/subalfred/pull/308".into(),
				labels: vec![
					Label { name: "Feat".into() },
					Label { name: "Core".into() },
					Label { name: "CLI".into() }
				]
			},
			PullRequest {
				title: "Update all docs".into(),
				html_url: "https://github.com/hack-ink/subalfred/pull/309".into(),
				labels: vec![Label { name: "Doc".into() }]
			},
			PullRequest {
				title: "Release `v0.9.0-rc18`".into(),
				html_url: "https://github.com/hack-ink/subalfred/pull/310".into(),
				labels: vec![Label { name: "R".into() }]
			}
		]
	);
}
