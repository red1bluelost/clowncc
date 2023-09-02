mod versioned;

use synstructure::decl_derive;

decl_derive!([Versioned, attributes(versioned)] => versioned::versioned);
