//! Algolia Object Id trait
/// All object passed to `update_document` or `insert_document` method of `Client` object must impl
/// AlgoliaObject trait
pub trait AlgoliaObject: serde::Serialize {
    /// Client calls this function to get the object Id of the document
    /// By default algolia itself provides Object id for each document getting inserted
    /// but this behaviour can be overridden by implementing AlgoliaObject 'get_object_id' method
    /// # Examples
    /// ```ignore
    ///    use EasyAlgolia::traits::AlgoliaObject;
    ///    use serde::Serialize ;
    ///    #[derive(Serialize)]
    ///    struct MyObject{
    ///      name:String,
    ///      location:String,
    ///      age:i32
    ///    }
    ///    
    ///  fn main(){
    ///    
    ///    // if you want default object from Algolia for your object
    ///    impl AlgoliaObject for MyObject ;
    ///     
    ///    // if you want a custom object id for your object
    ///    
    ///    impl AlgoliaObject for MyObject {
    ///       fn get_object_id(&self) -> Option<String> {
    ///          get_my_object_id(&self)
    ///          
    ///     }
    /// }  
    /// ```
    fn get_object_id(&self) -> String {
        "".into()
    }
}

/// yet to be documented
pub struct Sort {
    pub(crate) sort: String,
}
/// All the object passed into sort function of `SearchQuery` must impl this
/// yet to be documented
pub trait QuerySort {
    fn get_query(&self) -> &str;
}

impl QuerySort for Sort {
    fn get_query(&self) -> &str {
        &self.sort
    }
}
/// yet to be documented
pub trait Order {
    fn desc(&mut self) -> Sort;
    fn asc(&mut self) -> Sort;
}
impl Order for &str {
    fn desc(&mut self) -> Sort {
        Sort {
            sort: format!("{}:desc", self),
        }
    }
    fn asc(&mut self) -> Sort {
        Sort {
            sort: format!("{}:asc", self),
        }
    }
}

impl AlgoliaObject for serde_json::value::Value {}

/// This module provides a struct and methods for constructing search queries with
/// basic boolean operators, filters, and sorting options.
///
/// It's designed to encapsulate search query logic and construction for potential
/// integration with external search services or internal search implementations.
///

/// # Examples
///
/// ```rust
/// use algoliaobject::*;  
///
/// let query = SearchQuery::new()
///     .query("book")
///     .or_query("novel")
///     .filter("genre:fiction")
///     .sort("published_date".desc());
///
/// // Use the generated query string for your search implementation
/// println!("Constructed query: {}", query.to_string());
pub struct SearchQuery {
    is_query_set: bool,
    query: String,
    filter: Option<String>,
    pub(crate) sort: Option<String>,
}

// impl std::fmt::Display for SearchQuery {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.build_query())
//     }
// }

impl SearchQuery {

/// yet to be documented
    pub fn new() -> Self {
        Self {
            query: "".into(),
            filter: None,
            is_query_set: false,
            sort: None,
        }
    }

    /// yet to be documented
    pub fn query(mut self, query: &str) -> Self {
        self.is_query_set = true;
        self.query = query.into();
        self
    }
    /// yet to be documented
    pub fn or_query(mut self, query: &str) -> Self {
        self.is_query_set = true;
        if self.query.len() != 0 {
            self.query.push_str(&format!("|{query}"));
        } else {
            self.query.push_str(query);
        }

        self
    }
    /// yet to be documented
    pub fn and_query(mut self, query: &str) -> Self {
        self.is_query_set = true;
        if self.query.len() != 0 {
            self.query.push_str(&format!("&{query}"));
        } else {
            self.query.push_str(query);
        }

        self
    }

    /// yet to be documented
    pub fn set_query(&mut self) -> &str {
        self.query = self.query.replace("|", " OR ");
        self.query = self.query.replace("&", " AND ");
        &self.query
    }

    pub fn filter(mut self, filter: &str) -> Self {
        if let Some(ref mut s_filter) = self.filter {
            *s_filter = filter.into();
        } else {
            self.filter = Some(String::from(filter));
        }
        self
    }

    /// yet to be documented
    pub fn or_filter(mut self, filter: &str) -> Self {
        if let Some(ref mut s_filter) = self.filter {
            s_filter.push_str(&format!("|{filter}"));
        } else {
            self.filter = Some(String::from(filter));
        }
        self
    }

    /// yet to be documented
    pub fn and_filter(mut self, filter: &str) -> Self {
        if let Some(ref mut s_filter) = self.filter {
            s_filter.push_str(&format!("&{filter}"));
        } else {
            self.filter = Some(String::from(filter));
        }
        self
    }

    /// yet to be documented
    pub fn set_filter(&mut self) -> &str {
        if let Some(ref mut s_filter) = self.filter {
            *s_filter = s_filter.replace("|", " OR ");
            *s_filter = s_filter.replace("&", " AND ");
            s_filter
        } else {
            ""
        }
    }

    /// yet to be documented
    pub fn sort(mut self, order: impl QuerySort) -> Self {
        if let Some(ref mut sort) = self.sort {
            sort.push_str(&format!(",{}", order.get_query()))
        } else {
            self.sort = Some(order.get_query().into())
        }
        self
    }

    /// yet to be documented
    pub fn build_query(&mut self) -> String {
        self.set_query();
        self.set_filter();
        let mut query: String = "".into();
        if self.is_query_set {
            query.push_str(&format!("query={}", self.query));
        }
        // let query = &mut self.query;
        if let Some(ref s_filter) = self.filter {
            if self.is_query_set {
                query.push_str(&format!("&filter={s_filter}"))
            } else {
                query.push_str(&format!("filter={s_filter}"))
            }
        }
        if let Some(ref sort) = self.sort {
            if self.is_query_set {
                query.push_str(&format!("&sort={sort}"))
            } else {
                query.push_str(&format!("sort={sort}"))
            }
        }
        query
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_query_builder() {
        let mut query = SearchQuery::new()
            .query("orange")
            .or_query("banana")
            .and_query("mango");
        assert!(String::from("orange OR banana AND mango").eq(query.set_query()));
    }
    #[test]
    fn test_filter_builder() {
        let mut query = SearchQuery::new()
            .filter("name:faizal")
            .or_filter("price:>10")
            .and_filter("language:rust");

        assert!(String::from("name:faizal OR price:>10 AND language:rust").eq(query.set_filter()));
    }

    #[test]
    fn test_sort_builder() {
        let mut query = SearchQuery::new()
            .sort("price".desc())
            .sort("launch_date".asc())
            .sort("name".desc());
        assert!(String::from("sort=price:desc,launch_date:asc,name:desc").eq(&query.build_query()))
    }

    #[test]
    fn build_query_test() {
        let mut query = SearchQuery::new()
            .query("phone")
            .or_query("laptop")
            .filter("brand:apple")
            .and_filter("color:red")
            .sort("cost".desc())
            .sort("hype".asc());
        //dbg!(query.build_query());
        assert!(
            String::from("query=phone OR laptop&filter=brand:apple AND color:red&sort=cost:desc,hype:asc")
                .eq(&query.build_query())
        )
    }
}
