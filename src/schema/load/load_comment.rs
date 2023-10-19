use crate::comment::Comment;

pub(super) fn load_comment(comment: Option<&teo_parser::ast::comment::Comment>) -> Option<Comment> {
    comment.map(|comment| Comment {
        name: comment.name.clone(),
        desc: comment.desc.clone(),
    })
}