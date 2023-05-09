use std::io::{BufReader, Read, Cursor};

/**
 * Parsing context
 *
 * Provides context data and temporary storage during parsing operation.
 */
use crate::{journal::Journal, post::Post, xact::Xact};

pub(crate) struct ParsingContext {
    // pub reader: BufReader<T>,
    pub journal: Journal,

    /// Transaction being parsed currently. If exist, we are in the process of parsing posts.
    pub xact: Option<Xact>,
    pub posts: Option<Vec<Post>>,
}

impl ParsingContext {
    pub fn new() -> Self {
        Self {
            // commodity_pool: CommodityPool::new(),
            journal: Journal::new(),
            // cache:
            xact: None,
            posts: None,
            // reader: BufReader::new(Cursor::new("".to_string())),
        }
    }

    pub fn add_post(&mut self, post: Post) {
        if self.posts.is_none() {
            self.posts = Some(vec![]);
        }

        // self.posts.as_mut().and_then(|mut vec| vec.push(post));

        let col = self.posts.as_mut().unwrap();
        col.push(post);
    }
}

#[cfg(test)]
mod tests {
    use crate::post::Post;

    use super::ParsingContext;

    #[test]
    fn test_adding_posts() {
        let post = Post::new("acct", None);
        let mut context = ParsingContext::new();

        context.add_post(post);

        let col = context.posts.take().unwrap();
        assert_eq!(1, col.len());
    }
}
