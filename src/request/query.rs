use std::collections::HashMap;

type QueryParams = HashMap<String, String>;

pub struct Query {
  path: String,
  params: QueryParams,
}

impl Query {
  pub fn new() -> Self {
    Query {
      path: String::new(),
      params: HashMap::new(),
    }
  }

	pub fn path(&self) -> String {
		self.path.clone()
	}

	pub fn params(&self) -> QueryParams {
		self.params.clone()
	}

  pub(crate) fn parse(s: &str) -> Self {
    let q = s
      .split(" ")
      .filter(|part| part.starts_with("/"))
      .map(|str| urlencoding::decode(str).unwrap().to_string())
      .collect::<Vec<String>>()
      .first()
      .unwrap()
      .to_owned();
    let mut query = Query::new();
    query.parse_path(q.as_str());
    query.parse_params(q.as_str());
    query
  }

  fn parse_params(&mut self, q: &str) {
    let mut query = q.split("?");
    query.next();
    if let Some(params) = query.next() {
      let params = params
        .split("&")
        .filter_map(Query::map_param)
        .collect::<HashMap<String, String>>();

      self.params = params;
    }
  }

  fn parse_path(&mut self, q: &str) {
    let mut query = q.split("?");
    self.path = query.next().unwrap().to_string();
  }

  fn map_param(param: &str) -> Option<(String, String)> {
    let mut param = param.split("=");
    let key = param.next().unwrap().to_owned();
    if let Some(value) = param.next() {
      return Some((key, value.to_owned()));
    }
    return None;
  }
}
