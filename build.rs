fn main() {
    glib_build_tools::compile_resources(
        &["assets/templates"],
        "assets/templates/resources.gresource.xml",
        "assets_templates.gresource"
    )
}