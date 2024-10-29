// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct VerifyCustomHostnameOutput {
	#[allow(missing_docs)] // documentation missing in model
	pub body: std::option::Option<std::string::String>,
}
impl VerifyCustomHostnameOutput {
	#[allow(missing_docs)] // documentation missing in model
	pub fn body(&self) -> std::option::Option<&str> {
		self.body.as_deref()
	}
}
impl std::fmt::Debug for VerifyCustomHostnameOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut formatter = f.debug_struct("VerifyCustomHostnameOutput");
		formatter.field("body", &self.body);
		formatter.finish()
	}
}
/// See [`VerifyCustomHostnameOutput`](crate::output::VerifyCustomHostnameOutput)
pub mod verify_custom_hostname_output {
	/// A builder for [`VerifyCustomHostnameOutput`](crate::output::VerifyCustomHostnameOutput)
	#[non_exhaustive]
	#[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
	pub struct Builder {
		pub(crate) body: std::option::Option<std::string::String>,
	}
	impl Builder {
		#[allow(missing_docs)] // documentation missing in model
		pub fn body(mut self, input: impl Into<std::string::String>) -> Self {
			self.body = Some(input.into());
			self
		}
		#[allow(missing_docs)] // documentation missing in model
		pub fn set_body(mut self, input: std::option::Option<std::string::String>) -> Self {
			self.body = input;
			self
		}
		/// Consumes the builder and constructs a [`VerifyCustomHostnameOutput`](crate::output::VerifyCustomHostnameOutput)
		pub fn build(self) -> crate::output::VerifyCustomHostnameOutput {
			crate::output::VerifyCustomHostnameOutput { body: self.body }
		}
	}
}
impl VerifyCustomHostnameOutput {
	/// Creates a new builder-style object to manufacture [`VerifyCustomHostnameOutput`](crate::output::VerifyCustomHostnameOutput)
	pub fn builder() -> crate::output::verify_custom_hostname_output::Builder {
		crate::output::verify_custom_hostname_output::Builder::default()
	}
}