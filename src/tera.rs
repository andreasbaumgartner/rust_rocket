use rocket_dyn_templates::{context, tera::Tera, Template};

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "base.html",
        context! {
            title: "About",
            name: "Testing",
        },
    )
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Index",
            name: "Testing",
            items: vec!["One", "Two", "Three"],
        },
    )
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template(
        "base.html",
        r#"
        {% extends "base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
              <p>Here's some content for the about page.</p>
              <p>{{name}}</p>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}
