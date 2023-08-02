use super::method::Method;
use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

pub struct Request {
	path: String,
	query_string: Option<String>,
	method: Method,
}

impl TryFrom<&[u8]> for Request {

	type Error = ParseError;

	fn try_from(buffer: &[u8]) -> Result<Self, Self::Error>{
		// match from_utf8(buffer) {
		// 	Ok(request) => request,
		// 	Err(_) => return Err(ParseError::InvalidEncoding),
		// };

		// match from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
		// 	Ok(request) => request,
		// 	Err(e) => return Err(e)
		// };
		//equvalent of above is ?

		let request = from_utf8(buffer)?;

		unimplemented!()
	}
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i,c) in request.chars().enumerate() {
		if c == ' ' {
			return Some((&request[..i], &request[i+1..]));
		}
	}
	unimplemented!()
}

pub enum ParseError {
	InvalidRequest,
	InvalidMethod,
	InvalidProtocol,
	InvalidEncoding
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl From<Utf8Error> for ParseError {
	fn from(_:Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidMethod => "Invalid Method",
			Self::InvalidProtocol => "Invalid Protocol",
			Self::InvalidRequest => "Invalid Request"
		}
	}
}

impl Error for ParseError {}