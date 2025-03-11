fn main() {
    glib_build_tools::compile_resources(
        &["templates/resources"],
        "templates/resources/resources.gresource.xml",
        "resources.gresource",
    );
}