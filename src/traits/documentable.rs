use inflector::Inflector;
use crate::comment::Comment;
use crate::traits::named::Named;

pub trait Documentable: Named {

    fn comment(&self) -> Option<&Comment>;

    fn kind(&self) -> &'static str;

    fn title(&self) -> String {
        if let Some(title) = self.comment().as_ref().map(|c| c.name.as_ref()).flatten() {
            title.clone()
        } else {
            self.name().to_sentence_case()
        }
    }

    fn title_word_case(&self) -> String {
        self.title().to_word_case()
    }

    fn desc(&self) -> String {
        if let Some(desc) = self.comment().as_ref().map(|c| c.desc.as_ref()).flatten() {
            desc.clone()
        } else {
            format!("This {} doesn't have a description.", self.kind())
        }
    }
}