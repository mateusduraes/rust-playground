// The kind of documents bellow can be used to generate docs using rustdoc


/// Line comments; document the next item
/** Block comments; document the next item */

//! Line comments; document the enclosing item
/*! Block comments; document the enclosing item !*/


/// This module contains tests; Outer comment
mod tests {

}

mod tests {
    //! This module contains tests; Inner comment

}