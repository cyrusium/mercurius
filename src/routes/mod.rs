// represents the routes of the application
// the api follows a simple pattern for its versioning
// In the event of a breaking API change, the API version is bumped
// Where each version is has a codename along with a version number

// The current version is v1, codenamed "apollo"
// The next version is v2, codenamed "helios"
// The version after that is v3, codenamed "migro"

pub mod v1;
pub mod v2; // maybe remove until its launch?
pub mod v3; // maybe remove until its launch?