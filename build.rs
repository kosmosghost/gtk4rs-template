fn main() {
    // actions
    glib_build_tools::compile_resources(
        "resources",
        "resources/resources.gresource.xml",
        "gtk4rs-template.gresource",
    );
}