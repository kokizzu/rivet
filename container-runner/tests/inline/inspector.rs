use super::*;

#[test]
fn matches_inspector_ui_paths() {
	assert!(is_inspector_ui_path("/inspector/ui/"));
	assert!(is_inspector_ui_path("/inspector/ui/index.html"));
	assert!(is_inspector_ui_path("/inspector/ui/assets/app.js"));
	assert!(is_inspector_ui_path("/inspector/ui/?foo=bar"));
}

#[test]
fn ignores_non_inspector_paths() {
	assert!(!is_inspector_ui_path("/"));
	assert!(!is_inspector_ui_path("/inspector"));
	assert!(!is_inspector_ui_path("/inspector/"));
	assert!(!is_inspector_ui_path("/game/state"));
	assert!(!is_inspector_ui_path("/metadata"));
	// Prefix must be at the start, not merely contained.
	assert!(!is_inspector_ui_path("/x/inspector/ui/"));
}

#[test]
fn response_is_html_hello_world_with_ready() {
	let resp = inspector_response();
	assert_eq!(resp.status, 200);
	assert_eq!(
		resp.headers.get("content-type").map(String::as_str),
		Some("text/html; charset=utf-8")
	);
	let body = String::from_utf8(resp.body.expect("body")).unwrap();
	assert!(body.contains("<h1>Hello World</h1>"));
	// The host validates ready with z.literal("ready") AND z.literal(1) for `v`.
	// Dropping `v` makes safeParse fail silently -> iframe never unhides.
	assert!(body.contains(r#"post({ type: "ready", v: 1 })"#));
	// The init payload field is `authToken`, not `auth`.
	assert!(body.contains("d.authToken"));
	assert!(
		!body.contains("d.auth "),
		"must not read the nonexistent `auth` field"
	);
}

/// We speak the tab protocol but advertise nothing, so the host renders this page
/// with no tab chips. Adding one later means pushing an `{id,label,icon}` entry.
#[test]
fn supports_tabs_but_advertises_none() {
	let body = String::from_utf8(inspector_response().body.expect("body")).unwrap();
	assert!(body.contains(r#"post({ type: "tabs-available", v: 1, tabs: [] })"#));
	// Nothing should declare a tab entry.
	assert!(!body.contains(r#"icon: ""#), "no tab should be advertised");
	// A custom tab would make the host navigate to /inspector/custom-tabs/{id}/,
	// which we do not serve. (The identifier appears in a comment, so assert on the
	// enabled form rather than the bare word.)
	assert!(!body.contains("isCustom: true"));
}

#[test]
fn base64url_is_url_safe_and_unpadded() {
	// Bytes chosen so std base64 would emit '+' and '/'.
	assert_eq!(base64url_nopad(&[0xfb, 0xff, 0xbf]), "-_-_");
	assert_eq!(base64url_nopad(&[0x03]), "Aw");
	assert_eq!(base64url_nopad(b"hello"), "aGVsbG8");
	// 32 random-length bytes -> 43 chars, no '=' padding.
	assert_eq!(base64url_nopad(&[0u8; 32]).len(), 43);
	assert!(!base64url_nopad(&[0u8; 32]).contains('='));
}

#[test]
fn matches_metadata_path() {
	assert!(is_metadata_path("/metadata"));
	assert!(is_metadata_path("/metadata/"));
	assert!(is_metadata_path("/metadata?x=1"));
}

#[test]
fn ignores_non_metadata_paths() {
	assert!(!is_metadata_path("/api/rivet/metadata"));
	assert!(!is_metadata_path("/metadata/extra"));
	assert!(!is_metadata_path("/"));
	assert!(!is_metadata_path("/inspector/ui/"));
}

#[test]
fn metadata_reports_version_at_or_above_2_3_0() {
	let r = metadata_response();
	assert_eq!(r.status, 200);
	let body = String::from_utf8(r.body.unwrap()).unwrap();
	assert!(body.contains("\"version\":\"2.3.0\""), "body: {body}");
	assert!(body.contains("\"runtime\":\"rivetkit\""));
	// The gateway injects CORS itself; we must not set it.
	assert!(
		!r.headers
			.keys()
			.any(|k| k.to_lowercase().contains("access-control"))
	);
}
