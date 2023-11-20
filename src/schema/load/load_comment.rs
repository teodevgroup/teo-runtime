use crate::comment::Comment;

pub(super) fn load_comment(comment: Option<&teo_parser::ast::doc_comment::DocComment>) -> Option<Comment> {
    comment.map(|comment| Comment {
        name: comment.name().map(ToOwned::to_owned),
        desc: comment.desc().map(ToOwned::to_owned),
    })
}