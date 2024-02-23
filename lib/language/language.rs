/// LanguageFn wraps a C function that returns a pointer to a tree-sitter grammer.
#[repr(transparent)]
pub struct LanguageFn(pub unsafe extern "C" fn() -> *const ());
