mod file_loading;
pub mod model;
pub use file_loading::obj::load_wavefront;

/*
Gonna try to implement a gltf loader again another time
The file will load into an asset struct that I plan to use
for other file types in the future
*/
